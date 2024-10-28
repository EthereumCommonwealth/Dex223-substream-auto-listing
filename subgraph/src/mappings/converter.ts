import { Address, log } from "@graphprotocol/graph-ts";

import { ERC20WrapperCreatedEvent } from "../pb/auto_listing/v1/ERC20WrapperCreatedEvent";
import { ERC223WrapperCreatedEvent } from "../pb/auto_listing/v1/ERC223WrapperCreatedEvent";

import { Token } from "../../generated/schema";

import { getTokenId } from "../utils";

function saveResult(addressERC20: Address, addressERC223: Address): void {
  const id = getTokenId(addressERC20, addressERC223);
  let token = Token.load(id);
  if (token == null) {
    token = new Token(id);
    token.inConverter = true;
  }
}

export function handleConverterEventERC20(
  event: ERC20WrapperCreatedEvent
): void {
  const addressERC20 = Address.fromString(event.addressERC20);
  const addressERC223 = Address.fromString(event.addressERC223);
  saveResult(addressERC20, addressERC223);
}

export function handleConverterEventERC223(
  event: ERC223WrapperCreatedEvent
): void {
  const addressERC20 = Address.fromString(event.addressERC20);
  const addressERC223 = Address.fromString(event.addressERC223);
  saveResult(addressERC20, addressERC223);
}
