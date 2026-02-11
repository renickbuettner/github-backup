# How to Generate a GitHub Personal Access Token

To use this backup tool, you need to create a GitHub Personal Access Token (PAT) with appropriate permissions.

## Steps to Generate a Token

### For Classic Tokens

1. **Go to GitHub Settings**
   - Navigate to [GitHub.com](https://github.com)
   - Click on your profile picture in the top-right corner
   - Select **Settings**

2. **Access Developer Settings**
   - Scroll down in the left sidebar
   - Click on **Developer settings** (at the bottom)

3. **Generate New Token**
   - Click on **Personal access tokens** → **Tokens (classic)**
   - Click on **Generate new token** → **Generate new token (classic)**

4. **Configure Token Settings**
   - **Note**: Give your token a descriptive name (e.g., "GitHub Backup Tool")
   - **Expiration**: Choose an expiration period (recommended: 90 days or custom)
   
5. **Select Scopes**
   For backing up your repositories, select the following scopes:
   - ✅ `repo` - Full control of private repositories
     - This includes: `repo:status`, `repo_deployment`, `public_repo`, `repo:invite`, `security_events`
   
   For backing up organization repositories, you may also need:
   - ✅ `read:org` - Read org and team membership, read org projects

6. **Generate and Copy Token**
   - Scroll to the bottom and click **Generate token**
   - **IMPORTANT**: Copy the token immediately - you won't be able to see it again!
   - Store it securely (e.g., in a password manager)

### For Fine-Grained Tokens (Beta)

1. Follow steps 1-2 above
2. Click on **Personal access tokens** → **Fine-grained tokens**
3. Click **Generate new token**
4. Configure:
   - **Token name**: Descriptive name
   - **Expiration**: Set appropriate expiration
   - **Repository access**: 
     - Select "All repositories" to backup all your repos
     - Or select specific repositories you want to backup
   - **Permissions**:
     - Under "Repository permissions":
       - **Contents**: Read-only access
       - **Metadata**: Read-only access (automatically included)

5. Click **Generate token** and copy it immediately

## Using the Token

### Option 1: Environment Variable (Recommended)

```bash
export GITHUB_TOKEN="your_token_here"
```

Then run the backup tool without the `--token` flag:

```bash
cargo run --release -- --owner yourusername
```

### Option 2: Command Line Argument

```bash
cargo run --release -- --token your_token_here --owner yourusername
```

### Option 3: Using Mise

Set the environment variable in your shell, then:

```bash
mise run run
```

## Security Best Practices

⚠️ **NEVER commit your token to version control!**

- Store tokens securely (use a password manager or environment variables)
- Set appropriate expiration dates
- Revoke tokens when no longer needed
- Use the minimum required scopes
- Rotate tokens regularly

## Revoking a Token

If your token is compromised or no longer needed:

1. Go to GitHub Settings → Developer settings → Personal access tokens
2. Find the token in the list
3. Click **Delete** or **Revoke**
4. Confirm the action

## Troubleshooting

### "Bad credentials" error
- Your token may have expired
- The token might be invalid or incorrectly copied
- Generate a new token and try again

### "Not Found" error
- You might not have access to the repository/organization
- Check that you selected the correct scopes when creating the token
- Verify the username/organization name is correct

### Rate limiting
- GitHub has API rate limits (5,000 requests/hour for authenticated requests)
- The tool handles pagination automatically
- If you hit the limit, wait an hour or use a different token
