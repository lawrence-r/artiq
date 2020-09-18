from artiq.language.core import syscall
from artiq.language.types import TInt32, TInt64, TList, TNone, TTuple


@syscall(flags={"nowrite"})
def rabi_write(data: TInt32) -> TNone:
    raise NotImplementedError("syscall not simulated")

@syscall(flags={"nowrite"})
def rabi_read() -> TInt32:
    raise NotImplementedError("syscall not simulated")


