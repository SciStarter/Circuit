import requests
import json
import uuid
from pprint import pprint

API_ROOT = "http://127.0.0.1:8000/api/v1/"


def authorize_partner(partner_uid: str, partner_secret: str) -> str:
    return (
        requests.post(
            f"{API_ROOT}partner/authorize",
            headers={"Content-Type": "application/json"},
            data=json.dumps({"uid": partner_uid, "secret": partner_secret}),
        )
        .json()
        .get("token")
    )


def create_opportunity(token: str, opp: dict) -> dict:
    return requests.post(
        f"{API_ROOT}opportunity/",
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {token}",
        },
        data=json.dumps(opp),
    ).json()


def update_opportunity(token: str, opp: dict) -> dict:
    return requests.put(
        f"{API_ROOT}opportunity/{opp['uid']}",
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {token}",
        },
        data=json.dumps(opp),
    ).json()


def withdraw_opportunity(token: str, opp: dict) -> dict:
    opp["withdrawn"] = True

    return requests.put(
        f"{API_ROOT}opportunity/{opp['uid']}",
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {token}",
        },
        data=json.dumps(opp),
    ).json()


def get_opportunity_full(token: str, uid: str) -> dict:
    return requests.get(
        f"{API_ROOT}opportunity/{uid}",
        headers={
            "Authorization": f"Bearer {token}",
        },
    ).json()


def get_opportunity_public(uid: str) -> dict:
    return requests.get(
        f"{API_ROOT}opportunity/{uid}",
    ).json()


def run_tests(partner_uid, partner_secret):
    token = authorize_partner(partner_uid, partner_secret)

    print("Authorization token:", token)

    opp = create_opportunity(
        token,
        # This is a very minimal opportunity entry.
        {
            "uid": str(uuid.uuid4()),
            "partner_name": "testclient",
            "title": "test opportunity",
            "partner_opp_url": "https://slashdot.org/",
        },
    )

    print("Created:")
    pprint(opp)

    opp = withdraw_opportunity(token, opp)

    print("Withdrawn:")
    pprint(opp)

    opp["withdrawn"] = False
    opp["tags"] = ["test", "some-other-tag"]

    # These fields are read-only, so updating them has no effect
    opp["partner"] = "c3631f72-e395-45e2-9e4f-c74c58a54bea"
    opp["accepted"] = True

    opp = update_opportunity(token, opp)

    print("Updated:")
    pprint(opp)

    # Default status should be accepted for new opportunities
    assert opp["accepted"] == True

    # Verifying that the read-only fields have not changed
    assert opp["partner"] == partner_uid

    print("Get own:")
    pprint(get_opportunity_full(token, opp["uid"]))

    print("Get public:")
    pprint(get_opportunity_public(opp["uid"]))


if __name__ == "__main__":
    with open("secrets/testid.txt", "r") as f:
        partner_uid, partner_secret = [x.strip() for x in f.readlines()]
    run_tests(partner_uid, partner_secret)
