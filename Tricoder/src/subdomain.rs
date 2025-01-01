use crate::{
    model::{CrtShEntry, SubDomain},
    Error,
};
use reqwest::blocking::Client;
use std::{collections::HashSet, time::Duration};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

/// enumerate: Enumerate target subdomains using crt.sh
pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<SubDomain>, Error> {
    let url = format!("https://crt.sh/?q=%25.{}&output=json", target);
    // trick: JSON response from crt.sh can be deserialized into Vec<CrtShEntry>
    let response: Vec<CrtShEntry> = http_client.get(&url).send()?.json()?;

    let mut subdomains: HashSet<String> = response
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains("*"))
        .collect();

    // ensure the target domain only appears once
    subdomains.insert(target.to_string());

    // filter out subdomains that do not resolve to an IP address
    let subdomains: Vec<SubDomain> = subdomains
        .into_iter()
        .map(|domain| SubDomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter(resolves)
        .collect();

    Ok(subdomains)
}

/// resolves: Check if a subdomain resolves to an IP address
pub fn resolves(domain: &SubDomain) -> bool {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    let dns_resolver = Resolver::new(ResolverConfig::default(), opts)
        .expect("subdomain resolver: building DNS client");
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolves() {
        let subdomain = SubDomain {
            domain: "kerkour.com".to_string(),
            open_ports: Vec::new(),
        };
        assert!(resolves(&subdomain));
    }

    #[test]
    fn test_enumerate() {
        let http_client = Client::new();
        let subdomains = enumerate(&http_client, "kerkour.com").unwrap();
        let subdomains: HashSet<String> = subdomains
            .into_iter()
            .map(|subdomain| subdomain.domain)
            .collect();
        assert!(subdomains.contains("kerkour.com"));
    }
}
