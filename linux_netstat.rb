$prg_cache = {}

def read_proc_entry(proc_entry)
  Dir.glob("#{proc_entry}/fd/[0-9]*") do |fd_entry|
    begin
      fd_link = File.readlink(fd_entry)
      if fd_link.start_with?("socket")
        cmdline = File.open("#{proc_entry}/cmdline") { |f| f.gets }
        inode = fd_link.scan(/[0-9]+/).first
        $prg_cache[inode] = cmdline
      end
    rescue => err
      p err
    end
  end
end

def read_proc_net_tcp
  File.open("/proc/net/tcp") do |f|
    header = f.gets
    p header

    f.each_line do |line|
      fields = line.split(" ")
      if fields[0] == "sl"
        next
      end
      local_addr, local_port = fields[1].split(":")
      inode = fields[9]
      p "#{$prg_cache[inode]}: Port #{local_port.to_i(16)}"
    end
  end
end

def load_prg_cache
  Dir.glob("/proc/[0-9]*").each do |proc_entry|
    read_proc_entry proc_entry
  end
end

load_prg_cache
read_proc_net_tcp
