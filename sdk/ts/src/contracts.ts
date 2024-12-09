import { ethers } from 'ethers'

let trustedFowarderAbi = [
  {
    type: 'constructor',
    inputs: [{ name: 'name', type: 'string', internalType: 'string' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'eip712Domain',
    inputs: [],
    outputs: [
      { name: 'fields', type: 'bytes1', internalType: 'bytes1' },
      { name: 'name', type: 'string', internalType: 'string' },
      { name: 'version', type: 'string', internalType: 'string' },
      { name: 'chainId', type: 'uint256', internalType: 'uint256' },
      { name: 'verifyingContract', type: 'address', internalType: 'address' },
      { name: 'salt', type: 'bytes32', internalType: 'bytes32' },
      { name: 'extensions', type: 'uint256[]', internalType: 'uint256[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'execute',
    inputs: [
      {
        name: 'request',
        type: 'tuple',
        internalType: 'struct ERC2771Forwarder.ForwardRequestData',
        components: [
          { name: 'from', type: 'address', internalType: 'address' },
          { name: 'to', type: 'address', internalType: 'address' },
          { name: 'value', type: 'uint256', internalType: 'uint256' },
          { name: 'gas', type: 'uint256', internalType: 'uint256' },
          { name: 'deadline', type: 'uint48', internalType: 'uint48' },
          { name: 'data', type: 'bytes', internalType: 'bytes' },
          { name: 'signature', type: 'bytes', internalType: 'bytes' },
        ],
      },
    ],
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    name: 'executeBatch',
    inputs: [
      {
        name: 'requests',
        type: 'tuple[]',
        internalType: 'struct ERC2771Forwarder.ForwardRequestData[]',
        components: [
          { name: 'from', type: 'address', internalType: 'address' },
          { name: 'to', type: 'address', internalType: 'address' },
          { name: 'value', type: 'uint256', internalType: 'uint256' },
          { name: 'gas', type: 'uint256', internalType: 'uint256' },
          { name: 'deadline', type: 'uint48', internalType: 'uint48' },
          { name: 'data', type: 'bytes', internalType: 'bytes' },
          { name: 'signature', type: 'bytes', internalType: 'bytes' },
        ],
      },
      { name: 'refundReceiver', type: 'address', internalType: 'address payable' },
    ],
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    name: 'nonces',
    inputs: [{ name: 'owner', type: 'address', internalType: 'address' }],
    outputs: [{ name: '', type: 'uint256', internalType: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'verify',
    inputs: [
      {
        name: 'request',
        type: 'tuple',
        internalType: 'struct ERC2771Forwarder.ForwardRequestData',
        components: [
          { name: 'from', type: 'address', internalType: 'address' },
          { name: 'to', type: 'address', internalType: 'address' },
          { name: 'value', type: 'uint256', internalType: 'uint256' },
          { name: 'gas', type: 'uint256', internalType: 'uint256' },
          { name: 'deadline', type: 'uint48', internalType: 'uint48' },
          { name: 'data', type: 'bytes', internalType: 'bytes' },
          { name: 'signature', type: 'bytes', internalType: 'bytes' },
        ],
      },
    ],
    outputs: [{ name: '', type: 'bool', internalType: 'bool' }],
    stateMutability: 'view',
  },
  { type: 'event', name: 'EIP712DomainChanged', inputs: [], anonymous: false },
  {
    type: 'event',
    name: 'ExecutedForwardRequest',
    inputs: [
      { name: 'signer', type: 'address', indexed: true, internalType: 'address' },
      { name: 'nonce', type: 'uint256', indexed: false, internalType: 'uint256' },
      { name: 'success', type: 'bool', indexed: false, internalType: 'bool' },
    ],
    anonymous: false,
  },
  {
    type: 'error',
    name: 'ERC2771ForwarderExpiredRequest',
    inputs: [{ name: 'deadline', type: 'uint48', internalType: 'uint48' }],
  },
  {
    type: 'error',
    name: 'ERC2771ForwarderInvalidSigner',
    inputs: [
      { name: 'signer', type: 'address', internalType: 'address' },
      { name: 'from', type: 'address', internalType: 'address' },
    ],
  },
  {
    type: 'error',
    name: 'ERC2771ForwarderMismatchedValue',
    inputs: [
      { name: 'requestedValue', type: 'uint256', internalType: 'uint256' },
      { name: 'msgValue', type: 'uint256', internalType: 'uint256' },
    ],
  },
  {
    type: 'error',
    name: 'ERC2771UntrustfulTarget',
    inputs: [
      { name: 'target', type: 'address', internalType: 'address' },
      { name: 'forwarder', type: 'address', internalType: 'address' },
    ],
  },
  { type: 'error', name: 'FailedCall', inputs: [] },
  {
    type: 'error',
    name: 'InsufficientBalance',
    inputs: [
      { name: 'balance', type: 'uint256', internalType: 'uint256' },
      { name: 'needed', type: 'uint256', internalType: 'uint256' },
    ],
  },
  {
    type: 'error',
    name: 'InvalidAccountNonce',
    inputs: [
      { name: 'account', type: 'address', internalType: 'address' },
      { name: 'currentNonce', type: 'uint256', internalType: 'uint256' },
    ],
  },
  { type: 'error', name: 'InvalidShortString', inputs: [] },
  { type: 'error', name: 'StringTooLong', inputs: [{ name: 'str', type: 'string', internalType: 'string' }] },
]

export function trustedForwarderContract(address: string, signer: ethers.Signer) {
  return new ethers.Contract(address, trustedFowarderAbi, signer)
}
