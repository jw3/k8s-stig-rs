an experiment in stig scans
===

1. probably not a good idea
2. continue on
3.
    ```
    $ target/debug/k8s-stig 
    ✅ CNTR-K8-000220 pass
    ❌ CNTR-K8-002620 basic-auth-file is set
    Error: "1 failures"
    $ echo $?
    1
    ```

### expect
- to be using microk8s
