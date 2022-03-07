#!/bin/bash
if test -z "$1"; then
    kubectl exec --stdin --tty `kubectl get pod -l app=circuit-api-beta -o name` -- /bin/bash
else
    kubectl exec --stdin --tty "$1" -- /bin/bash;
fi
