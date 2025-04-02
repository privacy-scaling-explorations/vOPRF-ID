import requests

# Read the proof file
with open("../zk/oprf_commitment/target/proof", 'rb') as f:
    proof = f.read()

# Send POST request
response = requests.post(
    "http://localhost:8080/api/v1/evaluate",
    json={"proof": list(proof)}
)

print("Response:", response.json()) 