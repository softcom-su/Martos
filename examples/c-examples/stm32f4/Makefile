TOOLCHAIN = arm-none-eabi-

# Cortex-M4 target architecture with FPU
CFLAGS_PLATFORM = -mthumb -mcpu=cortex-m4 -mfpu=fpv4-sp-d16 -mfloat-abi=hard
LDFLAGS_PLATFORM = $(CFLAGS_PLATFORM)

CC = $(TOOLCHAIN)gcc
LD = $(TOOLCHAIN)ld

# Linker script location
LDSCRIPT = ld/stm32f4.ld

CFLAGS += -Wall -Werror -std=gnu11 -nostdlib $(CFLAGS_PLATFORM)
CFLAGS += -fno-strict-aliasing
CFLAGS += -fdata-sections -ffunction-sections
CFLAGS += -Os -g
CFLAGS += -Wa,--noexecstack

LDFLAGS += -nostdlib -T $(LDSCRIPT)
LDFLAGS += -Wl,-Map=$@.map -Wl,--cref -Wl,--gc-sections
LDFLAGS += $(LDFLAGS_PLATFORM)
LDFLAGS += -lgcc

# Link with martos
LDFLAGS += -Ltarget/thumbv7em-none-eabihf/release
LDFLAGS += -lstm32f429_static_lib

# Set C source files
C_SRC += src/main.c
C_SRC += src/startup.c
OBJS += $(C_SRC:.c=.o)

TARGET = main.elf

.PHONY: all clean

all: $(TARGET)

# Rules to build files
%.o: %.c
	$(CC) -c $(CFLAGS) $< -o $@

$(TARGET): $(OBJS)
	$(CC) $^ $(LDFLAGS) -o $@

# Target to clean build artifacts
clean:
	rm -f $(OBJS)
	rm -f $(TARGET) $(TARGET).map
