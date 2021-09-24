#!/bin/bash
source secrets/database.env
kubectl exec --stdin --tty `kubectl get pod -l app=postgres-beta -o name` -- /usr/bin/pg_dump -c -d ${DATABASE_URL}

