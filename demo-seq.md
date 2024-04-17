# Demo Sequence

|Pipeline/DevOps|Admin|Comments|
|---|---|---|
|mkdir admin-tool-testing; cd admin-tool-testing||DevOps performs setup in the pipeline|
|sf project generate -n att; cd att||
|sf org create scratch -f config/project-scratch-def.json -a att_sorg3 -v mohan.chinnappan.n_ea2@gmail.com||
||sf force org open -u att_sorg3|Admin Opens the Org and logs in, usually via Org URL|
||Admin Performs the required changes and updates spreadsheet (Quip) - the Source for Stories.csv||
|Get Triggered by the commit of the Stories.csv. Pipeline reads the Stories.csv to get the Org, Username, Story, ParentBranch, Items, DoneFlag|||
|Filters in for the Given Org and DoneFlag == False|||
|Writes the query.soql|||
|Runs: sf data query -f query.soql -o ORG -t -r csv|||
|git branch {Story}|||
|sf project retrieve start -o ORG|||
|git add -A|||
|git commit -m {Story}-{Description}|||
|git checkout {ParentBranch}|||
|git merge {Story}-{Description}||| 




