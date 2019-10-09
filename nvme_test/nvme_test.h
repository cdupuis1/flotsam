#pragma once

#include <linux/nvme.h>
#include "nvme.h"

#define	BACKING_STORE_SIZE	(100 * 1024 * 1024)

struct nvme_test_dev {
    void *store; /* Pointer to backing memory*/
    struct device dev;
    struct nvme_ctrl ctrl;
};