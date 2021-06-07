import struct

FORMAT = 'llHHI'
EVENT_SIZE = struct.calcsize(FORMAT)

in_file = open('/dev/input/event3', "rb")

event = in_file.read(EVENT_SIZE)
typed = ""

while event:
    (_, _, type, code, value) = struct.unpack(FORMAT, event)
    print(value)
