#include <stdio.h>
#include <unistd.h>
#include <linux/input.h>
#include <fcntl.h>

#define NUM_EVENTS 10

int main(void) {
  int fd = open("/dev/input/event3", O_RDONLY);
  int event_size = sizeof(struct input_event);
  int read_size = 0;
  int i;
  int read_count = 0;
  struct input_event events[NUM_EVENTS];
  while (1) {
    read_size = read(fd, events, event_size * NUM_EVENTS);
    for (i = 0; i < (read_size / event_size); i++) {
      printf("Code %d: %d\n", read_count, events[i].code);
    }
    read_count++;
  }
  return 0;
}
