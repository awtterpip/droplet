# droplet
A simple utility to assist small organizations/groups in distributing a service across multiple machines

### Note
Droplet is a very simple tool! It does not provide load-balancing or anything as such - it's designed for only one host to be active at a time.

## Usage
```
droplet [CONFIG]
```
\[CONFIG\] is an optional path to your service's `droplet.toml`.
If omitted, Droplet will search the current directory for it by default.

## Configuration
Create a `droplet.toml` in your service's root directory:
```toml
dns_code = "YourCodeHere" # optional

[service]
exec = "./path/to/script"
args = [
    "--flag",
    "argument"
]
```

## FreeDNS Dynamic DNS
You can provide a single domain name for your service, regardless of the public IP address of the current host - completely free!

This is possible by using FreeDNS and its Dynamic DNS feature.

### Step 1
Register an account with FreeDNS: https://freedns.afraid.org

### Step 2
Register a subdomain for your service: https://freedns.afraid.org/subdomain/

### Step 3
Enable dynamic updates v2 for your subdomain: https://freedns.afraid.org/dynamic/v2/

### Step 4
Copy your code from your Dynamic Update URL (e.g. ht<span>tp://sync.afraid.org/u/ThisIsYourCode/)

Then paste it into your service's `droplet.toml`:
```toml
dns_code = "YourCodeHere"
```

### Done!
Droplet will now automatically update the IP address your subdomain points to whenever you start a new Droplet instance!
