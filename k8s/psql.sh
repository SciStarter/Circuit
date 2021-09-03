#!/bin/bash
source secrets/database.env
kubectl exec --stdin --tty `kubectl get pod -l app=postgres-beta -o name` -- /usr/bin/psql ${DATABASE_URL}

