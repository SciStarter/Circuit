#!/bin/bash
kubectl exec --stdin --tty `kubectl get pod -l app=circuit-api-beta -o name` -- /usr/local/bin/toolkit "$@"
