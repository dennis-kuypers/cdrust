# cwk

A workflow helper. This is tightly integrated with our current workflow around pivotaltracker, git and github.

## Goals

### Initiate working on a new ticket

1. ensure that the ticket has the required workflow steps
2. ensure the ticket is in the correct state (started)
3. create and check out a local branch `PT-<pivotal_ticket_id>-<some_text>`
4. set the git commit message template to include `[#<pivotal_ticket_id>]`
5. record the fact that we are working on given ticket (or infer information from branch?)


### Drop out of workspace

1. clear/reset commit message template