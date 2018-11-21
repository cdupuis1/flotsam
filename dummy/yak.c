#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Yakko Warner");
MODULE_DESCRIPTION("Yak on, Yak off, the Yakker");
MODULE_VERSION("1");

static int __init yak_example_init(void) {
	printk(KERN_INFO "Yak On!\n");
	return 0;
}

static void __exit yak_example_exit(void) {
	printk(KERN_INFO "Yak Off!\n");
}

module_init(yak_example_init);
module_exit(yak_example_exit);
