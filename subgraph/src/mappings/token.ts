import { BigInt, Address, log } from "@graphprotocol/graph-ts";

import { TokenListed as TokenListedEvent } from "../pb/auto_listing/v1/TokenListed";
import { Token as TokenProto } from "../pb/auto_listing/v1/Token";
import { TokenInfo as TokenInfoProto } from "../pb/auto_listing/v1/TokenInfo";

import { AutoListing, Token, TokenListed } from "../../generated/schema";

import { ZERO_BI, ONE_BI } from "../constants";
import { getTokenId, getTokenListedId } from "../utils";

export function handleTokenListed(event: TokenListedEvent): void {
  const _token = event.token as TokenProto;
  const _tokenInfo = _token.tokenInfo as TokenInfoProto;

  if (_token == null || _tokenInfo == null) {
    return;
  }
  const addressERC20 = Address.fromString(_token.addressERC20);
  const addressERC223 = Address.fromString(_token.addressERC223);
  const listedBy = Address.fromString(event.autoListing);
  const timestamp = BigInt.fromI64(event.timestamp);
  const id = getTokenId(addressERC20, addressERC223);

  let token = Token.load(id);
  if (token == null) {
    token = new Token(id);
    token.addressERC20 = addressERC20.toHexString();
    token.addressERC223 = addressERC223.toHexString();
    token.decimals = BigInt.fromI64(_tokenInfo.decimals);
    token.symbol = _tokenInfo.symbol;
    token.inConverter = _tokenInfo.inConverter as boolean;
    token.name = _tokenInfo.name;
    token.numberAdditions = ZERO_BI;
  }
  token.numberAdditions = token.numberAdditions.plus(ONE_BI);
  let autoListing = AutoListing.load(listedBy.toHexString());
  if (autoListing == null) {
    log.error("AutoListing not found for address {}", [listedBy.toHexString()]);
    return;
  }
  autoListing.totalTokens = autoListing.totalTokens.plus(ONE_BI);
  autoListing.lastUpdated = timestamp;
  const listedById = getTokenListedId(addressERC20, addressERC223, listedBy);
  let tokenListed = TokenListed.load(listedById);
  if (tokenListed == null) {
    tokenListed = new TokenListed(listedById);
    tokenListed.timestamp = timestamp;
    tokenListed.token = token.id;
    tokenListed.authListing = autoListing.id;
  }

  token.save();
  tokenListed.save();
  autoListing.save();
}
