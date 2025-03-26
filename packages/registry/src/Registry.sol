// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Registry {
    // Mapping from node address to their public key (x,y coordinates)
    mapping(address => bytes32[2]) public nodePublicKeys;

    // Counter for number of registered nodes
    uint256 public numRegisteredNodes;

    // Event emitted when a node registers their public key
    event NodeRegistered(address indexed node, bytes32[2] publicKey);

    // Error when trying to register twice
    error AlreadyRegistered();
    // Error when trying to register more than 3 nodes
    error MaxNodesReached();

    /**
     * @notice Register a public key for an OPRF node
     * @param publicKey The public key as [x, y] coordinates
     */
    function register(bytes32[2] calldata publicKey) external {
        // Check if node is already registered
        if (nodePublicKeys[msg.sender][0] != 0 || nodePublicKeys[msg.sender][1] != 0) {
            revert AlreadyRegistered();
        }

        // Check if we've reached max nodes
        if (numRegisteredNodes >= 3) {
            revert MaxNodesReached();
        }

        // Store the public key
        nodePublicKeys[msg.sender] = publicKey;
        numRegisteredNodes++;

        emit NodeRegistered(msg.sender, publicKey);
    }

    /**
     * @notice Get the public key for a registered node
     * @param node The address of the node
     * @return The public key as [x, y] coordinates
     */
    function getNodePublicKey(address node) external view returns (bytes32[2] memory) {
        return nodePublicKeys[node];
    }

    /**
     * @notice Check if a node is registered
     * @param node The address of the node
     * @return True if the node is registered
     */
    function isRegistered(address node) external view returns (bool) {
        return nodePublicKeys[node][0] != 0 || nodePublicKeys[node][1] != 0;
    }
}
