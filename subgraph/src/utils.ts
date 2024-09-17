import { Address } from "@graphprotocol/graph-ts";

export function getTokenId(
  tokenAddressERC20: Address,
  tokenAddressERC223: Address
): string {
  return `${tokenAddressERC20.toHexString()}-${tokenAddressERC223.toHexString()}`;
}

export function getTokenListedId(
  tokenAddressERC20: Address,
  tokenAddressERC223: Address,
  listedBy: Address
): string {
  return `${getTokenId(
    tokenAddressERC20,
    tokenAddressERC223
  )}-${listedBy.toHexString()}`;
}
