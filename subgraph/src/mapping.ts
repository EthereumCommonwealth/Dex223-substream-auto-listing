import {
  log,
  Address,
  BigDecimal,
  BigInt,
  Bytes,
  store,
} from "@graphprotocol/graph-ts";
import { Events } from "./pb/auto_listing/v1/Events"; // 1.
import { ListingContractUpdated as ListingContractUpdatedEvent } from "./pb/auto_listing/v1/ListingContractUpdated"; // 1.
import { ListingPrice as ListingPriceEvent } from "./pb/auto_listing/v1/ListingPrice"; // 1.
import { TokenListed as TokenListedEvent } from "./pb/auto_listing/v1/TokenListed"; // 1
import { FeeToken as FeeTokenProto } from "./pb/auto_listing/v1/FeeToken"; // 1
import { Token as TokenProto } from "./pb/auto_listing/v1/Token"; // 1
import { TokenInfo as TokenInfoProto } from "./pb/auto_listing/v1/TokenInfo"; // 1

import {
  AutoListing,
  FeeToken,
  PriceDetail,
  Token,
  TokenListed,
} from "../generated/schema"; // 2.
import { Protobuf } from "as-proto/assembly";
import { ADDRESS_ZERO, ZERO_BI, ONE_BI } from "./constants"; // 3.
import { getTokenId, getTokenListedId } from "./utils"; // 4.

function handleListingContractUpdated(
  event: ListingContractUpdatedEvent
): void {
  const autoListingAddress = Address.fromString(event.autoListing);
  const metadata = Bytes.fromUint8Array(event.meta);
  const name = event.name;
  const owner = Address.fromString(event.owner);
  const url = event.url;
  const lastUpdated = BigInt.fromString(event.timestamp.toString());
  if (
    owner.equals(Address.fromString(ADDRESS_ZERO)) ||
    owner.equals(autoListingAddress) ||
    autoListingAddress.equals(Address.fromString(ADDRESS_ZERO))
  ) {
    log.error("Owner address is invalid {}", [owner.toHexString()]);
    return;
  }
  log.debug("AutoListing address {} and owner address {}", [
    autoListingAddress.toHexString(),
    owner.toHexString(),
  ]);

  let autoListing = AutoListing.load(autoListingAddress.toHexString());

  if (autoListing == null) {
    autoListing = new AutoListing(autoListingAddress.toHexString());
    autoListing.totalTokens = ZERO_BI;
    autoListing.name = name;
  }
  autoListing.lastUpdated = lastUpdated;
  autoListing.url = url;
  autoListing.meta = metadata;
  autoListing.owner = owner.toHexString();
  autoListing.save();
}

export function handleListingPrice(event: ListingPriceEvent): void {
  const _feeToken = event.feeToken as FeeTokenProto;
  const _tokenInfo = _feeToken.tokenInfo as TokenInfoProto;
  // log.error("FeeToken: {}  name: {} symbol: {} decimals: {}", [
  //   _feeToken.address,
  //   _tokenInfo.name,
  //   _tokenInfo.symbol,
  //   _tokenInfo.decimals.toString(),
  // ]);
  if (_feeToken == null || _tokenInfo == null) {
    return;
  }

  const _feeTokenAddress = Address.fromString(_feeToken.address);
  const priceValue = BigInt.fromString(event.price.toString());
  const autoListingAddress = Address.fromString(event.autoListing);
  const priceDetailId = `${autoListingAddress.toHexString()}-${_feeTokenAddress.toHexString()}`;

  let autoListing = AutoListing.load(autoListingAddress.toHexString());

  if (autoListing == null) {
    log.error("AutoListing not found for address {}", [
      autoListingAddress.toHexString(),
    ]);
    return;
  }
  let feeToken = FeeToken.load(_feeTokenAddress.toHexString());
  if (feeToken == null) {
    feeToken = new FeeToken(_feeTokenAddress.toHexString());
    feeToken.address = _feeTokenAddress.toHexString();
    feeToken.name = _tokenInfo.name;
    feeToken.symbol = _tokenInfo.symbol;
    feeToken.decimals = BigInt.fromI64(_tokenInfo.decimals);
    feeToken.inConverter = _tokenInfo.inConverter as boolean;
    feeToken.save();
  }

  let priceDetail = PriceDetail.load(priceDetailId);

  if (priceDetail == null) {
    priceDetail = new PriceDetail(priceDetailId);
    priceDetail.autoListing = autoListing.id;
  }

  if (priceValue.equals(ZERO_BI)) {
    // Remove row PriceDetail
    store.remove("PriceDetail", priceDetailId);

    // Remove realstionship for this PriceDetail in array priceDetail schema AutoListing
    autoListing.save();

    return;
  }
  priceDetail.price = priceValue;
  priceDetail.feeTokenAddress = feeToken.id; // Find schema Token id

  priceDetail.save();
  autoListing.save();
}

function handleTokenListed(event: TokenListedEvent): void {
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
    token.addressERC20 = _token.addressERC20;
    token.addressERC223 = _token.addressERC223;
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

export function handleEvents(bytes: Uint8Array): void {
  const eventsProto: Events = Protobuf.decode<Events>(bytes, Events.decode);
  const listingContractUpdateds = eventsProto.listingContractUpdateds;
  const listingPrice = eventsProto.listingPrices;
  const tokenListeds = eventsProto.tokenListeds;
  if (listingContractUpdateds.length > 0) {
    for (let i = 0; i < listingContractUpdateds.length; i++) {
      handleListingContractUpdated(listingContractUpdateds[i]);
    }
  }

  if (listingPrice.length > 0) {
    for (let i = 0; i < listingPrice.length; i++) {
      handleListingPrice(listingPrice[i]);
    }
  }
  if (tokenListeds.length > 0) {
    for (let i = 0; i < tokenListeds.length; i++) {
      handleTokenListed(tokenListeds[i]);
    }
  }
}
