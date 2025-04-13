import requests
import argparse

# Parse command line arguments
parser = argparse.ArgumentParser(description='Send proof to API endpoint')
parser.add_argument('--address', default='localhost', help='API server address (default: localhost)')
parser.add_argument('--port', default='8080', help='API server port (default: 8080)')
args = parser.parse_args()

# Read the proof file
with open("../zk/oprf_commitment/target/proof", 'rb') as f:
    proof = f.read()

# Construct the API URL
api_url = f"http://{args.address}:{args.port}/api/v1/evaluate"

# Send POST request
response = requests.post(
    api_url,
    json={"proof": list(proof)}
)

print("Response:", response.json()) 