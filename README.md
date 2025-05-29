# Arrest and release files in the CLI
### When you arrest a file, it is encrypted, and you got the key with you can "release" a file

Arrest example:
```bash
~$: cop ar example.txt
Arresting example.txt
Your key: 0eaf2H7JfWcA0t4D2Y7K8u3c1waC7eakco3Wcv6efo02338Sb768bO9F8A4f30ePbX7j9wfg98fO8P2V6x4can6p8R2s1V9B9q67aCdQdU7XfO4I8P3G41a57mam28fQ
```

Release example:
```bash
~$: cop rl example.txt
Enter key: # here you must enter a key, but its invisible
Releasing example.txt
```