Principle	              Behaviour

Namespace	              Can contain multiple patterns
Pattern ownership	      Last matching declaration wins
Folder patterns	        Independent collection
File patterns	          Independent collection
File vs folder          Independent resolution; memberships are combined 
Specificity	            No automatic specificity ranking
Undefined namespace	    Disallowed
Undefined dependency	  Disallowed

## Namespace Resolution

Namespace resolution determines which architectural namespaces apply to each project path.

A path can belong to multiple namespaces. File and folder namespace definitions are resolved independently, and their memberships are combined where applicable.

### Namespace Definitions

A namespace can be associated with multiple glob patterns.

Folder and file patterns are maintained as independent collections:

- `folders` defines folder namespace membership.
- `files` defines file namespace membership.

A file can belong to both a file namespace and a folder namespace. A directory can belong to a folder namespace.

### Pattern Ownership

When a path matches multiple namespace declarations within the same collection, the last matching declaration wins.

There is no automatic specificity ranking. Declaration order determines ownership.

File and folder namespace resolution are independent. A file may therefore receive both a file namespace and a folder namespace.

### Pattern Matching

Glob patterns determine namespace membership directly.

There is no additional folder inheritance or implicit ancestor-based membership.

A path belongs to a namespace only when its own path matches one of that namespace's configured patterns.

For example, `src/**` can match files and folders beneath `src`. Those paths receive the namespace because they match the glob, not because they inherit it from the `src` directory.

### Combined Membership

A file's resolved namespace memberships consist of:

- Its matching file namespace, if one exists.
- Its matching folder namespace, if one exists.

The memberships are combined rather than allowing one classification to override the other.

A namespace is included only once in the resulting membership collection.

### Undefined Membership

Paths that do not match any namespace remain unclassified.

Unclassified files and dependencies are not implicitly allowed. Dependencies involving undefined namespaces are disallowed.