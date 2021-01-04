# Stripe Intervation Service


[![Crates.io](https://img.shields.io/crates/d/stripe-integration?style=flat-square)](https://crates.io/crates/stripe-integration) [![Docker Pulls](https://img.shields.io/docker/pulls/fgribreau/stripe-integration)](https://hub.docker.com/r/fgribreau/stripe-integration) [![Get help on Codementor](https://cdn.codementor.io/badges/get_help_github.svg)](https://www.codementor.io/francois-guillaume-ribreau?utm_source=github&utm_medium=button&utm_term=francois-guillaume-ribreau&utm_campaign=github)  [![available-for-advisory](https://img.shields.io/badge/available%20for%20advising-yes-ff69b4.svg?)](http://bit.ly/2c7uFJq) ![extra](https://img.shields.io/badge/actively%20maintained-yes-ff69b4.svg?) [![Slack](https://img.shields.io/badge/Slack-Join%20our%20tech%20community-17202A?logo=slack)](https://join.slack.com/t/fgribreau/shared_invite/zt-edpjwt2t-Zh39mDUMNQ0QOr9qOj~jrg)

> Easiest stripe integration (ever) for Rapid SaaS development & deployment

## Features

- Launch the app and get [Stripe Checkout](https://stripe.com/docs/payments/checkout) for your SaaS (upcoming: Stripe Portals)
- Fully customizable through [environment variables](./.envrc.default)
- [Sentry](https://sentry.io/) reporting

## How to use

Link your website pricing page to `https://app.tld.com/v1.0/checkout/go/{stripe_pricing_id}` where app.tld.com is where the current app is deployed.

 
## Who use this?

- [Image-Charts](https://www.image-charts.com) - Chart API for image generation
