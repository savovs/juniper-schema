# juniper-schema
Tries to show a nice example of structuring a real-world GraphQL app in Rust using the Juniper crate.
The idea is that you have multiple GraphQL schemas in different folders, separated by concern.
They are all merged at the end to form one root schema. This would make it easy to add new GraphQL functionality without having everything in one big file.

