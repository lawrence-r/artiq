from artiq.language.core import *
from artiq.language.types import *


@syscall(flags={"nowrite"})
def hal_write(data: TInt32) -> TNone:
    raise NotImplementedError("syscall not simulated")

@syscall(flags={"nowrite"})
def hal_read() -> TInt32:
    raise NotImplementedError("syscall not simulated")


class Hal:
    def __init__(self, dmgr, core_device="core"):
        self.core = dmgr.get(core_device)

    @kernel
    def write(self, value):
        hal_write(value)

    @kernel
    def read(self):
        return hal_read()

