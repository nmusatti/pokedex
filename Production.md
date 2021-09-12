### Production grade Pokedex

To discuss in detail how Pokedex could be improved to make it ready for production would probably
take a whole book. In his "The Mythical Man Month" Frederick Brooks argues that to turn a working
program into a product takes nine times the effort required to write the original program. Many
things have changed since he wrote the book, and so probably have the numbers, but it is still a
major task.

I will try to describe a few areas that, in my opinion, would require some form of intervention.
I assume that the deployment platform is Kubernetes and that the corresponding infrastructure
is already available. I consider networking issues, such as routing, DNS and proxies, to be outside
the scope of this essay.

### Development

While Pokedex's logic is rather simple and it is unlikely to impact performance in a significant way,
more attention should be devoted to the impact of coding choices on performance, from ownership
vs. borrowing, to which resources might be shared among requests, for instance to better exploit
connection pooling, and so on.

Error handling should be given more thought, striving for minimality but ensuring that information
is provided where it is needed, be it to support problem recovery or to provide better feedback to
client applications.

Testing should be more thorough and it should cover both internal application logic and any
assumption made on the behaviour of the systems Pokedex interacts with. Some form of contract
testing, e.g. tests implemented as API clients, should be adopted to ensure that the API keeps
behaving as promised to its consumers. Tests that interact with external systems should be separated
from tests that do not, be it because they verify isolated components or because stubs or mocks are
used. At least this second group of tests should be easily executed by developers and possibly be a
precondition to committing to version control. Full tests should be run periodically and as a
precondition to committing to the main branch.

Pokedex should also be stress tested by submitting to it automatically generated heavy loads. This
may require stubbing out or mocking external interfaces, as they may consider such abnormal load as
malicious.

The development environment should provide preventive measures, such as code formatting and linting
as commit hooks, and behaviour feedback in the form of automated testing. The adoption of code
reviews as a precondition to committing to the main branch should be considered.

### Deployment

In a Kubernetes deployment access to a service should be routed through an ingress, which should
only expose it as HTTPS. While Pokedex doesn't expose sensitive information, it might be desirable
to adopt TLS also for intra-cluster networking, in which case a service mesh, such as Istio or
Linkerd, may be of use. Care should be given to correctly sizing the number of replicas to be
executed. Some form of autoscaling should also be considered. If a high degree of availability is
required Pokedex's pods should be forced to run on more than one node, and possibly on nodes
residing in different data centres/availability zones. Overprovisioning may help reduce the impact
of partial outages.

Ideally Pokedex's Docker image should contain a default configuration suitable for execution on
developer workstations, which should be overridable from the outside environment, e.g. from 
environment variables. Different config maps could be used to provide suitable sets of variable
definitions for different environments. For services that may be considered to be conceptually part
of the same application as Pokedex I find it convenient to rely more on conventions, such as
consistent naming, than explicit configuration. In-cluster DNS support is very convenient in this
respect. Deployment progress through different environments, such as test, UAT and production,
should be automated and combine the same container image with specific configurations.

### Observability

Errors should be reported promptly and it should be possible to analyse what happened in order to
provide corrections in a timely manner. This may be achieved by application logs combined with some
form of event monitoring. Ideally this would both signal situations requiring swift intervention and
provide a backlog of upstanding and solved error conditions. different kind of notifications should
be used for problems of different severity. Automated paging may be set up for critical problems.
API clients should receive informative diagnostics that should enable their developers to provide
feedback to their own end users and to report problems effectively.

Performance should also be monitored, to ensure unexpected load spikes are dealt with rapidly and to
provide developers with information on areas that need further analysis and optimization. Data such
as request handling times, memory and cpu usage should be collected and be made available through
tools such as the ELK stack or the Prometheus/Graphana combination, to enable both an overall view
and drilling down to specific events.

### Security

In addition to HTTPS and TLS stricter security requirements may need to be addressed, in the form of
authentication and authorization. Based on how critical the service is considered and on 
cross-service policies, password based authentication may be adopted, backed by traditional
providers such as FreeIPA or Active Directory. A more modern approch might entail the adoption of
protocols such as OpenID Connect. Authorization might be rely on custom solutions or protocol such 
as OAuth 2. It might prove necessary to identify end user for auditing purposes, or service accounts
may be considered sufficient.
