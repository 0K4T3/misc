def main():
    with open('/tmp/test_fifo') as fp:
        print(fp.read())


if __name__ == '__main__':
    main()
