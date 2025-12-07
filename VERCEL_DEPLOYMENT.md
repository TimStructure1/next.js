# Vercel Deployment Setup

This repository contains multiple Next.js applications that can be deployed to Vercel.

## Apps Available for Deployment

- **Documentation Site**: `apps/docs/` - Next.js documentation site
- **Bundle Analyzer**: `apps/bundle-analyzer/` - Bundle analysis UI

## Quick Start: Deploy Docs App from VS Code

### 1. Connect to Vercel

1. Go to [vercel.com](https://vercel.com) and sign in
2. Click "Add New Project"
3. Import this GitHub repository
4. Configure:
   - **Root Directory**: `apps/docs`
   - **Framework Preset**: Next.js
5. Click "Deploy"

### 2. Push from VS Code

Once connected, simply:

1. Make changes in VS Code
2. Commit (Ctrl+Enter in Source Control)
3. Push (click sync button or run `git push`)
4. Vercel deploys automatically! 🚀

## Detailed Instructions

See `apps/docs/DEPLOYMENT.md` for comprehensive deployment documentation including:

- Step-by-step setup guide
- Vercel CLI usage
- Troubleshooting tips
- VS Code integration

## Configuration Files

- **`apps/docs/vercel.json`**: Deployment configuration for the docs app
- **`vercel.json`** (root): GitHub CI settings only

## Notes

- This is a monorepo managed by pnpm and Turborepo
- Each app in `apps/` can be deployed separately
- Set the correct Root Directory in Vercel project settings
