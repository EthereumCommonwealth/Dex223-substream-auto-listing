import { BigInt, Address, store, log } from "@graphprotocol/graph-ts";

import { ListingPrice as ListingPriceEvent } from "../pb/auto_listing/v1/ListingPrice";
import { FeeToken as FeeTokenProto } from "../pb/auto_listing/v1/FeeToken";
import { TokenInfo as TokenInfoProto } from "../pb/auto_listing/v1/TokenInfo";

import { AutoListing, FeeToken, PriceDetail } from "../../generated/schema";

import { ZERO_BI } from "../constants";

export function handleListingPrice(event: ListingPriceEvent): void {
  const feeTokenEvent = event.feeToken as FeeTokenProto;
  const tokenInfo = feeTokenEvent.tokenInfo as TokenInfoProto;
  if (feeTokenEvent == null || tokenInfo == null) {
    return;
  }
  const feeTokenAddress = Address.fromString(feeTokenEvent.address);
  const priceValue = BigInt.fromString(event.price.toString());
  const autoListingAddress = Address.fromString(event.autoListing);

  const priceDetailId = `${autoListingAddress.toHexString()}-${feeTokenAddress.toHexString()}`;

  let autoListing = AutoListing.load(autoListingAddress.toHexString());

  if (autoListing == null) {
    log.error("AutoListing not found for address {}", [
      autoListingAddress.toHexString(),
    ]);
    return;
  }
  let feeToken = FeeToken.load(feeTokenAddress.toHexString());
  if (feeToken == null) {
    feeToken = new FeeToken(feeTokenAddress.toHexString());
    feeToken.address = feeTokenAddress.toHexString();
    feeToken.name = tokenInfo.name;
    feeToken.symbol = tokenInfo.symbol;
    feeToken.decimals = BigInt.fromI64(tokenInfo.decimals);
    feeToken.inConverter = tokenInfo.inConverter as boolean;
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
