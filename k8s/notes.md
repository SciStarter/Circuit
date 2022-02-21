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

Create file secrets/jwt.env containing JWT_SIGNING_KEY, then

    kubectl create secret generic jwt-signing-beta --from-env-file=secrets/jwt.env

Create file secrets/logger.env containing LOGGER_ENDPOINT,
LOGGER_ACCESS_KEY, and LOGGER_SECRET containing the S3-compatible
endpoint URL, access key, and secret, then

    kubectl create secret generic logger-beta --from-env-file=secrets/logger.env

Create file secrets/airtable.env containing AIRTABLE_KEY containing
the API key for the info@scistarter.org Airtable account, then

    kubectl create secret generic airtable-beta --from-env-file=secrets/airtable.env

Create file secrets/opencage.env containing OPENCAGE_API_KEY
containing the api.opencage.com key, then

    kubectl create secret generic opencage --from-env-file=secrets/opencage.env

Create file secrets/mapbox.env containing MAPBOX_TOKEN
containing the mapbox.com key, then

    kubectl create secret generic mapbox --from-env-file=secrets/mapbox.env

Create file secrets/mailgun.env containing MAILGUN_ID and
MAILGUN_SECRET, then

    kubectl create secret generic mailgun --from-env-file=secrets/mailgun.env

Create file secrets/scistarter.env containing SNM_PAIR and SCI_PUB variables:
SNM_PAIR={"public":[47,144,119,108,27,231,194,169,221,12,38,107,60,198,185,120,166,116,21,98,251,47,117,114,55,11,173,249,64,38,161,85],"secret":[36,213,201,43,240,173,66,192,215,230,225,180,116,123,115,144,248,91,129,52,252,156,95,4,186,143,167,106,5,181,47,69]}
SCI_PUB={"public":[43,249,154,12,113,243,216,41,63,196,156,215,8,4,138,247,12,210,117,67,211,160,72,168,115,222,211,57,154,161,244,113]}

Not that the SNM_PAIR value listed here is valid, but it’s just an
example and should not be used in production. To generate a new key
pair, go to the api directory and run

    cargo run –bin keypair

You will also need to change public key used by SciStarter to the
public value from the new key pair.

Once you have the env file set up, do

    kubectl create secret generic scistarter --from-env-file=secrets/scistarter.env
