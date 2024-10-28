# Time Ticker Valiatation on a PREEMPT-RT Patched Ubuntu

100 us slot duration:

With thread priority setting and RT throttling `sudo sysctl -w kernel.sched_rt_runtime_us=-1`

```bash
$ cargo build --release && sudo ./target/release/rt_ticker
    Finished `release` profile [optimized] target(s) in 0.06s

Statistics over 100,000 slots
Mean jitter: 13 ns
Max jitter: 1567 ns
Median jitter: 13 ns
95th percentile: 25 ns
99th percentile: 25 ns
```

System infomation

```bash
$ neofetch
            .-/+oossssoo+/-.
        `:+ssssssssssssssssss+:`
      -+ssssssssssssssssssyyssss+-
    .ossssssssssssssssssdMMMNysssso.
   /ssssssssssshdmmNNmmyNMMMMhssssss/
  +ssssssssshmydMMMMMMMNddddyssssssss+
 /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/    amyang@ubuntu 
.ssssssssdMMMNhsssssssssshNMMMdssssssss.   --------------- 
+sssshhhyNMMNyssssssssssssyNMMMysssssss+   OS: Ubuntu 24.04.1 LTS x86_64 
ossyNMMMNyMMhsssssssssssssshmmmhssssssso   Kernel: 6.8.1-1010-realtime 
ossyNMMMNyMMhsssssssssssssshmmmhssssssso   Uptime: 24 mins 
+sssshhhyNMMNyssssssssssssyNMMMysssssss+   Packages: 2015 (dpkg), 13 (snap) 
.ssssssssdMMMNhsssssssssshNMMMdssssssss.   Shell: zsh 5.9 
 /sssssssshNMMMyhhyyyyhdNMMMNhssssssss/    Resolution: 1920x1080 
  +sssssssssdmydMMMMMMMMddddyssssssss+     Terminal: node 
   /ssssssssssshdmNNNNmyNMMMMhssssss/      CPU: AMD Ryzen 3 2300X (4) @ 3.500GHz 
    .ossssssssssssssssssdMMMNysssso.       GPU: NVIDIA GeForce RTX 2070 Rev. A 
      -+sssssssssssssssssyyyssss+-         Memory: 1352MiB / 7868MiB 
        `:+ssssssssssssssssss+:`
            .-/+oossssoo+/-.                                       
                                                                   
```