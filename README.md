# Snap URL

## Purpose

Sometimes you need to share some information with other person. You need to share the information privately and secure.

Snap URL generates one time use URL to access your information.

## How it works
You fill out the system form with the information you want to share. A payment data can be added for the paid service to get:
- notification that your information has been read
- time frame when the information will be available
You need to provide e-mail address to get a notification to, or the system can provide a check token with
a limited lifetime to read your message status. 
The access URL to your information is displayed after.
The access URL to your information you can distribute in any way you want. The system doesn't distribute the URL
and doesn't provide a security for a distribution.

## Paid features
1. define the link valid time interval and start time (free service - the link is valid immediately for a week)
2. get a notification that URL was opened and which IP from
3. revoke the URL

## Privacy
Snap URL doesn't collect an information about the users of the system besides of required to provide paid service.
Snap URL isn't responsible for what information is shared and doesn't store the information besides of required for the service.


## A protection from hacking or using this service for a SPAM
Snap URL uses the following technique. Any access URL has a response. However the response reflects the real data only
for the correct URL. Other responses contain some random data. There is no indication that the data were real or mocked up.
Since the real data returned by the system only once, multiple accesses of the correct URL will produce the actual data
only at first hit.
A chance of DOS reduced by obligatory throttling. Any response gets processed for 3-10 seconds in dependency on the 
current load of the system.

## Sizing
An information size is limited to 32K. No images, videos or sound can be in. However, an information can include links, or any other access tokens
in a text form. UTF-8 is used and an information is HTML formatted. There is also a protection against JavaScript code
execution in an information and direct or indirect accessing other URLs from it without a user consent.

## Dependencies
The Rust code has one penendency [simweb](https://github.com/vernisaz/simweb). It was tested only with TJWS CGI extension.