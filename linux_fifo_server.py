import json
import os
import time


def main():
    if not os.path.exists('/tmp/test_fifo'):
        os.mkfifo('/tmp/test_fifo')
    with os.open('/tmp/test_fifo', mode='w') as fp:
        while True:
            fp.write(json.dumps({'hoge': 'fuga'}))
            time.sleep(5)


if __name__ == '__main__':
    main()
