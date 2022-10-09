require 'socket'

def main
  count = 1

  loop do
    puts "Send packet #{count}"

    sock = UDPSocket.new
    sock.connect('localhost', 80)
    sock.send('Hello, World!', 0)
    sock.close

    sleep(1)
    count += 1
  end
end

main
