// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Registry {
    // Fixed-size array of public keys (x,y coordinates)
    bytes32[2][3] public nodePublicKeys;

    // Number of registered nodes (0-3)
    uint256 public numRegisteredNodes;

    // Event emitted when a node registers their public key
    event NodeRegistered(uint256 indexed nodeId, bytes32[2] publicKey);

    // Error when trying to register more than 3 nodes
    error MaxNodesReached();
    // Error when trying to register a duplicate public key
    error DuplicatePublicKey();
    // Error when node ID is out of range
    error InvalidNodeId();

    /**
     * @notice Register a public key for an OPRF node
     * @param publicKey The public key as [x, y] coordinates
     * @return nodeId The ID of the registered node (0-2)
     */
    function register(bytes32[2] calldata publicKey) external returns (uint256 nodeId) {
        // Check if we've reached max nodes
        if (numRegisteredNodes >= 3) {
            revert MaxNodesReached();
        }

        // Check if this public key is already registered
        for (uint256 i = 0; i < numRegisteredNodes; i++) {
            if (nodePublicKeys[i][0] == publicKey[0] && nodePublicKeys[i][1] == publicKey[1]) {
                revert DuplicatePublicKey();
            }
        }

        // Store the public key
        nodeId = numRegisteredNodes;
        nodePublicKeys[nodeId] = publicKey;
        numRegisteredNodes++;

        emit NodeRegistered(nodeId, publicKey);
        return nodeId;
    }

    /**
     * @notice Get the public key for a registered node by ID
     * @param nodeId The ID of the node (0-2)
     * @return The public key as [x, y] coordinates
     */
    function getNodePublicKey(uint256 nodeId) external view returns (bytes32[2] memory) {
        if (nodeId >= numRegisteredNodes) {
            revert InvalidNodeId();
        }
        return nodePublicKeys[nodeId];
    }

    /**
     * @notice Check if a public key is registered
     * @param publicKey The public key to check
     * @return True if the public key is registered
     */
    function isRegistered(bytes32[2] calldata publicKey) external view returns (bool) {
        for (uint256 i = 0; i < numRegisteredNodes; i++) {
            if (nodePublicKeys[i][0] == publicKey[0] && nodePublicKeys[i][1] == publicKey[1]) {
                return true;
            }
        }
        return false;
    }
}
