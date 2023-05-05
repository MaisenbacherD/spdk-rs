
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wimplicit-function-declaration"

#include "nvme_helper.h"

#include <spdk/bdev_module.h>
#include <bdev/bdev_internal.h>
#include <spdk/nvme_spec.h>
#include <spdk/thread.h>

#pragma GCC diagnostic pop

uint32_t *
nvme_cmd_cdw10_get(struct spdk_nvme_cmd *cmd) {
       return &cmd->cdw10;
}

uint32_t *
nvme_cmd_cdw11_get(struct spdk_nvme_cmd *cmd) {
       return &cmd->cdw11;
}

uint32_t
nvme_cmd_cdw10_get_val(const struct spdk_nvme_cmd *cmd) {
       return cmd->cdw10;
}

uint32_t
nvme_cmd_cdw11_get_val(const struct spdk_nvme_cmd *cmd) {
       return cmd->cdw11;
}

struct spdk_nvme_status *
nvme_status_get(struct spdk_nvme_cpl *cpl) {
	return &cpl->status;
}

uint16_t *
nvme_status_raw_get(struct spdk_nvme_cpl *cpl) {
	return &cpl->status_raw;
}

/* Based on spdk_bdev_nvme_admin_passthru with the check for desc->write
 * removed.
 * spdk_bdev_nvme_io_passthru has a comment on parsing the command to
 * determine read or write. As we only have one user, just remove the check.
 */
int
spdk_bdev_nvme_admin_passthru_ro(struct spdk_bdev_desc *desc, struct spdk_io_channel *ch,
			      const struct spdk_nvme_cmd *cmd, void *buf, size_t nbytes,
			      spdk_bdev_io_completion_cb cb, void *cb_arg)
{
	struct spdk_bdev *bdev = spdk_bdev_desc_get_bdev(desc);
	struct spdk_bdev_io *bdev_io;
	struct spdk_bdev_channel *channel = spdk_io_channel_get_ctx(ch);

	bdev_io = bdev_channel_get_io(channel);
	if (!bdev_io) {
		return -ENOMEM;
	}

	bdev_io->internal.ch = channel;
	bdev_io->internal.desc = desc;
	bdev_io->type = SPDK_BDEV_IO_TYPE_NVME_ADMIN;
	bdev_io->u.nvme_passthru.cmd = *cmd;
	bdev_io->u.nvme_passthru.buf = buf;
	bdev_io->u.nvme_passthru.nbytes = nbytes;
	bdev_io->u.nvme_passthru.md_buf = NULL;
	bdev_io->u.nvme_passthru.md_len = 0;

	bdev_io_init(bdev_io, bdev, cb_arg, cb);

	bdev_io_submit(bdev_io);
	return 0;
}
