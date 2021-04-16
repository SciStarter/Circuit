# Install ingress-nginx

    kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/controller-v0.44.0/deploy/static/provider/cloud/deploy.yaml

See [the instructions](https://kubernetes.github.io/ingress-nginx/deploy/) for more information.

# Install cert-manager

    kubectl apply -f https://github.com/jetstack/cert-manager/releases/download/v1.2.0/cert-manager.yaml

It may be necessary to update the version number in the above. Check
[the installation instructions](https://cert-manager.io/docs/installation/kubernetes/)
for up-to-date information.

# Secrets

Create file secrets/database.env containing DATABASE_URL and
POSTGRES_PASSWORD, then

    kubectl create secret generic db-secret-beta --from-env-file=secrets/database.env

The DATABASE_URL value will be based on a URL as assigned by
kubernetes for a stateful service. The database.env file should look
something like this:

    DATABASE_URL=postgres://postgres:NoPe@postgres-beta/postgres
    POSTGRES_PASSWORD=NoPe

Create file secrets/superuser.env containing SUPERUSER_EMAIL and
SUPERUSER_PASSWORD, then

    kubectl create secret generic superuser-secret-beta --from-env-file=secrets/superuser.env

These settings are used to create a superuser account if it doesn’t
exist. It’s recommended to change the superuser password as one of
your first actions after deployment.
