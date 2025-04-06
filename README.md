# Snap URL

## Purpose

Sometimes you need to share information with another person. You need to share the information privately and secure.

Snap URL generates one time use URL to access the shared information.

## How it works
You fill out a form with the information you want to share. A payment data can be added for the paid service to get:
- notification that your information has been read
- time frame when the information will be available for reading
You need to provide e-mail address to get a notification to, or the system can provide a check token with
a limited lifetime to read your message status. 
The access URL to the information is displayed after.
The URL can be distributed in any way you desire. The system doesn't distribute the URL
and doesn't provide a security for distribution.

## Paid features
1. define the link valid time interval and start time (free service - the link is valid immediately for a week)
2. get a notification that URL was opened, when, and which IP from
3. revoke the URL

## Payment protection
It's required to provide an e-mail for a receipt.
When you pay for extra features, the following security measurements taking a place:
1. Payment receipt gets sent in the e-mail you provided, however if payment didn't get through, you receive nothing,
there is no other indication of an unsuccessful payment. You will get a snap URL in any case, but without extra features
 if the payment was unsuccessful
2. You'll receive a text message with a security token, when a phone number is provided with a payment
3. Compromised e-mail addresses didn't get accepted in payments, however here is no indication the provided e-mail address is compromised

No e-mail receipt means that something  wrong was with your payment.

## Privacy
Snap URL doesn't collect information about users of the system besides of required to provide the paid service.
Snap URL isn't responsible for what information is shared and doesn't store the information besides of required for the service.


## A protection from hacking or using this service for a SPAM, or other illegal activities
Snap URL uses the following technique. Any access URL has a response. However, the response reflects the real data only
for the correct URL. Other responses contain some random data. There is no indication that the data were real or mocked up.
Since the real data returned by the system only once, multiple accesses of the correct URL will produce the actual data
only at the first hit.
A chance of DoS attack reduced by an obligatory throttling. Any request gets processed for 3-10 seconds in dependency on the 
current load of the system. It also protects the system from flooding with dummy or oversized messages.

## Sizing
An information size is limited to 32K. No images, videos or sound can be in. However, an information can include links, or any other access tokens
in a text form. UTF-8 is used and an information is HTML formatted. There is also a protection against JavaScript code
execution in an information and direct or indirect accessing other URLs from it without a user consent.

## Dependencies
The Rust code has two dependencies [simweb](https://github.com/vernisaz/simweb) and [simjson](https://github.com/vernisaz/simjson).
It was tested only with [TJWS](https://tjws.sf.net/) CGI extension and
[SimHTTP](https://github.com/vernisaz/simhttp), however there
is a big chance it will work with any CGI capable Web Server.