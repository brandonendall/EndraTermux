/*
    This file is part of libnspire.

    libnspire is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    libnspire is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with libnspire.  If not, see <http://www.gnu.org/licenses/>.
*/

#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <ctype.h>

static long diag_ms(void) {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (long)(ts.tv_sec * 1000L + ts.tv_nsec / 1000000L);
}

#include <string.h>

#include "handle.h"
#include "error.h"
#include "data.h"
#include "service.h"
#include "dir.h"

static int dir_enum(nspire_handle_t *handle, struct nspire_dir_info **d) {
	int ret;
	char *name;
	uint32_t size, date;
	uint8_t is_dir;
	struct nspire_dir_info *new_dir;
	struct nspire_dir_item *current;

	unsigned char buffer[254];
	long started = diag_ms();
	size_t actual = 0;
	fprintf(stderr, "[V2 ENUM] stage=request_write item_index=%zu host_sid=%u\n", (*d)->num, (unsigned)handle->host_sid);

	if ( (ret = data_write8(handle, 0x0E)) ) {
		fprintf(stderr, "[V2 ENUM] stage=request_write_error rc=%d elapsed_ms=%ld\n", ret, diag_ms()-started);
		return ret;
	}
	fprintf(stderr, "[V2 ENUM] stage=response_wait elapsed_ms=%ld\n", diag_ms()-started);
	if ( (ret = data_read(handle, buffer, sizeof(buffer), &actual)) ) {
		fprintf(stderr, "[V2 ENUM] stage=response_error rc=%d elapsed_ms=%ld\n", ret, diag_ms()-started);
		return ret;
	}
	fprintf(stderr, "[V2 ENUM] stage=response_ok actual=%zu first=%02x second=%02x elapsed_ms=%ld\n", actual, actual ? buffer[0] : 0, actual > 1 ? buffer[1] : 0, diag_ms()-started);
	if (!actual) return -NSPIRE_ERR_INVALID;

	if (buffer[0] == 0xFF) {
		fprintf(stderr, "[V2 ENUM] stage=end_marker elapsed_ms=%ld\n", diag_ms()-started);
		return 1;
	}

	if (actual < 12) {
		fprintf(stderr, "[V2 ENUM] stage=short_response bytes=%zu\n", actual);
		return -NSPIRE_ERR_INVALID;
	}
	if ( (ret = data_scan("hbswwb0", buffer, actual,
			NULL, NULL, &name, &size, &date, &is_dir)) )
		return ret;

	if ((*d)->num >= 4096) {
		fprintf(stderr, "[V2 ENUM] stage=safety_limit count=%zu\n", (*d)->num);
		return -NSPIRE_ERR_INVALID;
	}
	new_dir = realloc(*d, sizeof(struct nspire_dir_info) +
			(((*d)->num + 1) * sizeof(struct nspire_dir_item)));
	if (!new_dir)
		return -NSPIRE_ERR_NOMEM;
	current = new_dir->items + new_dir->num;
	new_dir->num++;
	*d = new_dir;

	strncpy(current->name, name, sizeof(current->name));
	current->name[sizeof(current->name)-1] = '\0';
	current->size = size;
	current->date = date;
	current->type = is_dir;
	fprintf(stderr, "[V2 ENUM] stage=item_ok index=%zu name=%s type=%u size=%u elapsed_ms=%ld\n", new_dir->num, current->name, (unsigned)current->type, (unsigned)current->size, diag_ms()-started);

	return NSPIRE_ERR_SUCCESS;
}

int nspire_dirlist(nspire_handle_t *handle, const char *path,
		struct nspire_dir_info **info_ptr) {
	int ret;
	size_t len;
	uint8_t buffer[254], code;
	uint16_t result;
	struct nspire_dir_info *d;

	long started = diag_ms();
	const char *stage = "service_connect";
	fprintf(stderr, "[V2 FILE] path=%s stage=%s connected_before=%d\n", path, stage, handle->connected);
	if ( (ret = service_connect(handle, 0x4060)) )
		return ret;

	fprintf(stderr, "[V2 FILE] stage=service_connect_result ret=%d elapsed_ms=%ld\n", ret, diag_ms()-started);
	stage = "request_build";
	/* Begin dir enum */
	if ( (ret = data_build("bs0", buffer, sizeof(buffer), &len,
			0x0D, path)) )
		goto end;

	stage = "request_write";
	fprintf(stderr, "[V2 FILE] stage=%s request_bytes=%zu elapsed_ms=%ld\n", stage, len, diag_ms()-started);
	if ( (ret = data_write(handle, buffer, len)) )
		goto end;

	stage = "initial_response_read";
	fprintf(stderr, "[V2 FILE] stage=%s elapsed_ms=%ld\n", stage, diag_ms()-started);
	if ( (ret = data_read(handle, buffer, 2, NULL)) )
		goto end;

	fprintf(stderr, "[V2 FILE] stage=initial_response_received bytes=%02x:%02x elapsed_ms=%ld\n", buffer[0], buffer[1], diag_ms()-started);
	stage = "initial_response_decode";
	if ( (ret = data_scan("bb", buffer, sizeof(buffer),
			NULL, &code)) )
		goto end;

	switch (code) {
	case 0x0A:
		ret = -NSPIRE_ERR_NONEXIST;
		goto end;
	case 0x0F:
		ret = -NSPIRE_ERR_INVALID;
		goto end;
	}

	d = malloc(sizeof(struct nspire_dir_info));
	if (!d) {
		ret = -NSPIRE_ERR_NOMEM;
		goto end;
	}

	d->num = 0;
	*info_ptr = d;
	stage = "enumerate";
	/* Start enumerating */
	while (1) {
		ret = dir_enum(handle, info_ptr);
		if (ret < 0) {
			free(*info_ptr);
			*info_ptr = NULL;
			goto end;
		}

		if (ret)
			break;
	}

	stage = "finalize";
	/* End dir enum */
	if ( (ret = data_build("b", buffer, sizeof(buffer), &len,
			0x0F)) )
		goto end;

	if ( (ret = data_write(handle, buffer, len)) )
		goto end;

	if ( (ret = data_read(handle, buffer, 2, NULL)) )
		goto end;

	if ( (ret = data_scan("h", buffer, sizeof(buffer),
			NULL, &result)) )
		goto end;

	// TODO: result ignored here, what to do on failure anyway?

	ret = NSPIRE_ERR_SUCCESS;
end:
	fprintf(stderr, "[V2 FILE] stage=%s result=%d elapsed_ms=%ld connected=%d\n", stage, ret, diag_ms()-started, handle->connected);
	int disconnect_rc = service_disconnect(handle);
	fprintf(stderr, "[V2 FILE] stage=disconnect result=%d connected_after=%d elapsed_ms=%ld\n", disconnect_rc, handle->connected, diag_ms()-started);
	return ret;
}

void nspire_dirlist_free(struct nspire_dir_info *d) {
	free(d);
}

int nspire_dir_create(nspire_handle_t *handle, const char *path) {
	int ret;
	size_t len;
	uint16_t result;
	uint8_t buffer[254];

	if ( (ret = service_connect(handle, 0x4060)) )
		return ret;

	if ( (ret = data_build("hs", buffer, sizeof(buffer), &len,
			0x0A03, path)) )
		goto end;

	if ( (ret = data_write(handle, &buffer, len)) )
		goto end;

	if ( (ret = data_read(handle, &buffer, 2, NULL)) )
		goto end;

	if ( (ret = data_scan("h", buffer, sizeof(buffer), &result)) )
		goto end;

	ret = (result == 0xFF00) ? NSPIRE_ERR_SUCCESS : -NSPIRE_ERR_EXISTS;
end:
	service_disconnect(handle);
	return ret;
}

int nspire_dir_delete(nspire_handle_t *handle, const char *path) {
	int ret;
	size_t len;
	uint16_t result;
	uint8_t buffer[254];


	if ( (ret = service_connect(handle, 0x4060)) )
		return ret;

	if ( (ret = data_build("hs", buffer, sizeof(buffer), &len,
			0x0B03, path)) )
		goto end;

	if ( (ret = data_write(handle, &buffer, len)) )
		goto end;

	if ( (ret = data_read(handle, &buffer, 2, NULL)) )
		goto end;

	if ( (ret = data_scan("h", buffer, sizeof(buffer), &result)) )
		goto end;

	ret = (result == 0xFF00) ? NSPIRE_ERR_SUCCESS : -NSPIRE_ERR_NONEXIST;
end:
	service_disconnect(handle);
	return ret;
}

int nspire_attr(nspire_handle_t *handle, const char *path,
		struct nspire_dir_item *info) {
	int ret;
	size_t len;
	uint8_t is_dir, buffer[254];
	uint32_t size, date;

	if ( (ret = service_connect(handle, 0x4060)) )
		return ret;

	if ( (ret = data_build("hs0", buffer, sizeof(buffer), &len,
			0x2001, path)) )
		goto end;

	if ( (ret = data_write(handle, buffer, len)) )
		goto end;

	if ( (ret = data_read(handle, buffer, sizeof(buffer), NULL)) )
		goto end;

	if (buffer[0] != 0x20) {
		ret = -NSPIRE_ERR_NONEXIST;
		goto end;
	}

	if ( (ret = data_scan("bwwb0", buffer, sizeof(buffer),
			NULL, &size, &date, &is_dir)) )
		goto end;

	strncpy(info->name, path, sizeof(info->name));
	info->name[sizeof(info->name)-1] = '\0';
	info->size = size;
	info->date = date;
	info->type = is_dir;

	ret = NSPIRE_ERR_SUCCESS;
end:
	service_disconnect(handle);
	return ret;
}
