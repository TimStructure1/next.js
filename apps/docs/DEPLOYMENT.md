# Deploying Next.js Docs to Vercel from VS Code

This guide will help you deploy the Next.js documentation site to Vercel from VS Code.

## Prerequisites

1. **Vercel Account**: Create an account at [vercel.com](https://vercel.com)
2. **Vercel CLI** (optional but recommended): Install with `npm i -g vercel`
3. **Git configured**: Ensure your repository is connected to GitHub

## Method 1: Deploy via Git Push (Automatic)

This is the recommended method for continuous deployment.

### Step 1: Connect Repository to Vercel

1. Go to [vercel.com](https://vercel.com) and log in
2. Click "Add New Project"
3. Import your GitHub repository: `TimStructure1/next.js`
4. Configure the project:
   - **Framework Preset**: Next.js
   - **Root Directory**: `apps/docs`
   - **Build Command**: `pnpm run build-docs` (or leave default)
   - **Output Directory**: `.next` (or leave default)
   - **Install Command**: `pnpm install` (or leave default)

### Step 2: Enable Automatic Deployments

- Vercel will automatically deploy on every push to your main branch
- Pull request deployments are also created automatically

### Step 3: Push from VS Code

1. Make your changes in VS Code
2. Commit your changes (Ctrl+Enter in Source Control panel)
3. Push to GitHub (sync button or `git push`)
4. Vercel will automatically detect the push and deploy

## Method 2: Deploy via Vercel CLI

If you prefer manual deployments or want to deploy from VS Code terminal:

### Step 1: Install and Login

```bash
npm i -g vercel
vercel login
```

### Step 2: Link Project

From the `apps/docs` directory:

```bash
cd apps/docs
vercel link
```

Follow the prompts to link to your existing Vercel project.

### Step 3: Deploy

```bash
# Deploy to preview
vercel

# Deploy to production
vercel --prod
```

## Configuration Files

The following configuration files have been set up:

- **`apps/docs/vercel.json`**: Deployment configuration for the docs app
- **`apps/docs/.gitignore`**: Prevents build artifacts from being committed

## Troubleshooting

### Build Fails with Network Errors

If the build fails locally but works on Vercel, this is normal. The build process may:

- Fetch fonts from Google Fonts
- Download images from external URLs
- These operations require internet access that may be restricted locally

### Wrong Directory Deployed

Ensure in Vercel project settings:

- Root Directory is set to `apps/docs`
- Framework Preset is set to Next.js

### Environment Variables

If your app requires environment variables:

1. Go to Project Settings > Environment Variables in Vercel
2. Add your variables there
3. They will be available during build and runtime

## VS Code Integration

For seamless deployment from VS Code:

1. **Git Integration**: Use VS Code's built-in Source Control (Ctrl+Shift+G)
2. **Terminal**: Use integrated terminal (Ctrl+`) to run Vercel CLI commands
3. **Extensions**: Consider installing:
   - Vercel (official extension)
   - GitLens for enhanced Git features

## Deployment Status

After pushing, you can:

- View deployment status on [vercel.com](https://vercel.com)
- Get deployment URLs from Vercel dashboard
- Enable notifications for deployment status in VS Code with the Vercel extension

## Next Steps

1. Push this repository to GitHub
2. Connect it to Vercel as described above
3. Push any changes from VS Code to automatically deploy

Your site should now be live and automatically deploy on every push!
