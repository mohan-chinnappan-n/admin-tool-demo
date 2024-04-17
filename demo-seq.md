# Demo Sequence

|Pipeline/DevOps|Admin|Comments|
|---|---|---|
|mkdir admin-tool-testing; cd admin-tool-testing||DevOps performs setup in the pipeline|
|sf project generate -n att; cd att||
|sf org create scratch -f config/project-scratch-def.json -a att_sorg3 -v mohan.chinnappan.n_ea2@gmail.com||
||sf force org open -u att_sorg3|Admin Opens the Org and logs in, usually via Org URL|
||Admin Performs the required changes and updates spreadsheet (Quip) - the Source for StoryDoc.csv|
|Reads the StoryDoc.csv to get the Org, Username, Story, ParentBranch, Items, DoneFlag||
|Filters in for the Given Org and DoneFlag == False||

