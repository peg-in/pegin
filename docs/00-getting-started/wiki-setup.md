# 🐧 PEGIN Wiki Setup & Deployment Guide

## What You've Got

You now have a **complete, self-contained knowledge base** for PEGIN in 3 formats:

1. **HTML Wiki** (interactive, searchable)
   - File: `docs/wiki/PEGIN_Wiki_Knowledge_Base.html`
   - Best for: Browsing, sharing, offline access

2. **Topic folders** (markdown by subject)
   - See `docs/README.md` for the full index
   - Best for: Deep dives, version control, Obsidian/GitHub

3. **Consolidated reference**
   - File: `docs/wiki/PEGIN_Wiki.md`

---

## Option 1: Use the Wiki Immediately (Easiest)

### On Your Computer
```
1. Find: docs/wiki/PEGIN_Wiki_Knowledge_Base.html
2. Double-click it
3. Opens in your browser
4. Start reading!
```

**That's it.** No setup needed. Fully functional offline.

### Key Features
- ✅ Sidebar navigation
- ✅ Search function
- ✅ 14 main sections
- ✅ Works offline
- ✅ No internet required
- ✅ No installation needed

---

## Option 2: Host Online (For Sharing)

### A. GitHub Pages (Free, Easiest)

#### Step 1: Create GitHub Account (if you don't have one)
- Go to https://github.com
- Sign up (free)
- Verify email

#### Step 2: Create Repository
```bash
1. Click "+" in top right
2. Select "New repository"
3. Name: pegin-wiki (or whatever you like)
4. Description: "PEGIN Knowledge Base - Decentralized Enterprise Identity"
5. Public checkbox: CHECKED
6. Click "Create repository"
```

#### Step 3: Upload HTML File
```bash
1. Click "Add file" → "Upload files"
2. Drag PEGIN_Wiki_Knowledge_Base.html here
3. Or click to browse and select file
4. Commit message: "Add PEGIN wiki"
5. Click "Commit changes"
```

#### Step 4: Enable GitHub Pages
```bash
1. Go to Settings tab
2. Scroll to "Pages" section
3. Source: Select "main" branch
4. Select "/ (root)" folder
5. Click "Save"
6. Wait 1-2 minutes for deployment
```

#### Step 5: Access Your Wiki
```
Your wiki is now at:
https://yourusername.github.io/pegin-wiki/PEGIN_Wiki_Knowledge_Base.html

Share this link!
```

### B. Netlify (Free, Simple)

#### Step 1: Sign Up
- Go to https://netlify.com
- Click "Sign up"
- Use GitHub to sign up (easy)

#### Step 2: Deploy
```bash
1. Click "New site from Git"
2. Select GitHub
3. Find your pegin-wiki repository
4. Click to deploy
5. Done! Auto-deployed from GitHub
```

#### Step 3: Access Your Wiki
```
Your wiki is at:
https://your-site-name.netlify.app/PEGIN_Wiki_Knowledge_Base.html

Netlify gives you a custom domain option
```

### C. Your Own Server

If you have a web server:

```bash
# Upload the file
scp PEGIN_Wiki_Knowledge_Base.html user@yourserver.com:/var/www/html/

# Access it
https://yourserver.com/PEGIN_Wiki_Knowledge_Base.html
```

---

## Option 3: Advanced Setup (With Auto-Updates)

### Create a Complete GitHub Repository

#### Step 1: Clone Repository Locally
```bash
cd ~/projects
git clone https://github.com/yourusername/pegin-wiki.git
cd pegin-wiki
```

#### Step 2: Create Project Structure
```
pegin-wiki/
├── PEGIN_Wiki_Knowledge_Base.html    (main wiki)
├── docs/                              (markdown files)
│   ├── 01-home.md
│   ├── 02-business-principles.md
│   ├── 03-user-owned-login.md
│   └── ... (all 18 documents)
├── assets/                            (images, css, js)
├── README.md                          (top-level readme)
└── .gitignore                         (git ignore file)
```

#### Step 3: Copy All Files
```bash
# Copy all markdown files to docs/
cp PEGIN_*.md docs/

# Copy HTML wiki to root
cp PEGIN_Wiki_Knowledge_Base.html ./

# Copy README
cp PEGIN_Wiki_README.md README.md
```

#### Step 4: Commit & Push
```bash
git add .
git commit -m "Add complete PEGIN knowledge base"
git push origin main
```

#### Step 5: Create GitHub Pages Site
```
Go to Settings → Pages
- Source: Deploy from branch
- Branch: main
- Folder: / (root)
- Save

Your wiki is now live at:
https://yourusername.github.io/pegin-wiki/
```

---

## Option 4: Build a Full Documentation Site (Advanced)

### Using MkDocs (Python-based)

#### Step 1: Install MkDocs
```bash
pip install mkdocs mkdocs-material
```

#### Step 2: Create Project
```bash
mkdocs new pegin-wiki
cd pegin-wiki
```

#### Step 3: Configure mkdocs.yml
```yaml
site_name: PEGIN Knowledge Base
site_description: Decentralized Enterprise Identity Platform
theme:
  name: material
  palette:
    - scheme: default
      primary: purple
      accent: purple
  features:
    - navigation.instant
    - navigation.tracking
    - search.suggest

nav:
  - Home: index.md
  - Getting Started:
    - Quick Start: getting-started/quick-start.md
    - Glossary: getting-started/glossary.md
  - Core Concepts:
    - User-Owned Login: concepts/user-owned-login.md
    - Permission Management: concepts/permissions.md
    - DIG Network: concepts/dig-network.md
  - Architecture: architecture/overview.md
  - Business: business/model.md
  - Use Cases: use-cases/index.md

plugins:
  - search
  - awesome-pages
```

#### Step 4: Add Your Documents
```bash
# Copy all markdown files
cp ../PEGIN_*.md docs/

# Organize them in subdirectories
mkdir -p docs/{concepts,business,architecture,use-cases}
```

#### Step 5: Build & Deploy
```bash
# Local preview
mkdocs serve

# Deploy to GitHub Pages
mkdocs gh-deploy
```

#### Step 6: Access
```
Your site is now at:
https://yourusername.github.io/pegin-wiki/
```

---

## Wiki Management

### Updating the Wiki

#### If using HTML File (Easiest)
1. Open `PEGIN_Wiki_Knowledge_Base.html` in text editor
2. Find the section you want to update
3. Edit the content
4. Save the file
5. Refresh browser to see changes

#### If using GitHub + MkDocs
1. Update the markdown file in `docs/`
2. Commit and push
3. MkDocs auto-rebuilds
4. Changes live within 5 minutes

### Adding New Content

#### To HTML Wiki
```html
<!-- Find the appropriate section in the HTML -->
<!-- Add new section div -->
<div id="new-topic" class="section">
    <h1>New Topic Title</h1>
    <p>Content goes here...</p>
</div>

<!-- Add link in sidebar -->
<h2>Section Name</h2>
<a href="#" onclick="showSection('new-topic', event)">📝 New Topic</a>
```

#### To Markdown Docs
1. Create new `.md` file
2. Add to mkdocs.yml nav
3. Push to GitHub
4. Auto-deploys

---

## Accessing Your Wiki

### Direct Links
```
HTML Wiki (single file):
yoursite.com/PEGIN_Wiki_Knowledge_Base.html

MkDocs Site:
yoursite.com/pegin-wiki/

GitHub Pages:
github.com/yourusername/pegin-wiki

GitHub Repository:
github.com/yourusername/pegin-wiki/tree/main/docs
```

### Sharing the Wiki

#### Option A: Share Direct Link
```
"Read our knowledge base: https://yoursite.com/wiki"
```

#### Option B: Share HTML File
```
1. Attach PEGIN_Wiki_Knowledge_Base.html to email
2. Recipient opens in browser
3. Works offline
```

#### Option C: Embed in Your Website
```html
<!-- In your website, create an iframe -->
<iframe src="https://yoursite.com/PEGIN_Wiki_Knowledge_Base.html" 
        width="100%" 
        height="800px">
</iframe>
```

---

## Wiki Analytics (Optional)

### Track Wiki Usage

#### Option A: Google Analytics (Free)
```html
<!-- Add to the HTML head -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_ID"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA_ID');
</script>
```

#### Option B: Netlify Analytics (Free with Netlify)
- Automatically enabled
- View in Netlify dashboard

#### Option C: Simple View Counter
- Add a hit counter (no tracking)
- Shows total page views

---

## Backup & Maintenance

### Regular Backups
```bash
# Backup all documents
tar -czf pegin-wiki-backup-$(date +%Y%m%d).tar.gz docs/ *.html

# Store safely
# AWS S3, Google Drive, Dropbox, or local external drive
```

### Version Control
```bash
# Always use git for version control
git add .
git commit -m "Update documentation: [what changed]"
git push

# Easy to revert if needed
git log --oneline
git revert <commit-hash>
```

### Documentation Checklist
- [ ] All 18 markdown files present
- [ ] HTML wiki tested in browser
- [ ] Navigation works
- [ ] Search function works
- [ ] Links not broken
- [ ] Formatting correct
- [ ] Backed up to GitHub
- [ ] Deployed to web (if using online)

---

## Quick Deployment Checklist

### For HTML Wiki Only (5 minutes)
- [ ] Download `PEGIN_Wiki_Knowledge_Base.html`
- [ ] Open in browser
- [ ] Test navigation
- [ ] Test search
- [ ] Done!

### For GitHub Pages (15 minutes)
- [ ] Create GitHub account
- [ ] Create new repository
- [ ] Upload HTML file
- [ ] Enable GitHub Pages
- [ ] Share the link
- [ ] Done!

### For Full Documentation Site (30 minutes)
- [ ] Install MkDocs
- [ ] Create MkDocs project
- [ ] Configure mkdocs.yml
- [ ] Add all markdown files
- [ ] Deploy to GitHub Pages
- [ ] Test in browser
- [ ] Done!

---

## Troubleshooting

### Wiki Doesn't Load
**Problem:** Blank page when opening HTML
- **Solution:** Use a modern browser (Chrome, Firefox, Safari, Edge)
- **Solution:** Make sure JavaScript is enabled

### Search Doesn't Work
**Problem:** Search box doesn't filter menu
- **Solution:** JavaScript might be disabled - enable it
- **Solution:** Try a different browser

### GitHub Pages Not Showing
**Problem:** Repository created but no site
- **Solution:** Wait 5-10 minutes for GitHub to build
- **Solution:** Check Settings → Pages → branch is "main"
- **Solution:** Make sure file is in root directory

### Broken Links
**Problem:** Links between sections don't work
- **Solution:** Clear browser cache (Ctrl+Shift+Del)
- **Solution:** Try different browser

### Mobile Display Issues
**Problem:** Wiki doesn't look good on phone
- **Solution:** HTML wiki is responsive - should work on mobile
- **Solution:** Try landscape orientation
- **Solution:** Use zoom (pinch-zoom on mobile)

---

## Next Steps

### Immediate (Today)
1. ✅ Open the HTML wiki in your browser
2. ✅ Read the Home section
3. ✅ Test the search function
4. ✅ Familiarize yourself with content

### Short Term (This Week)
1. Create GitHub repository (if sharing)
2. Upload HTML + markdown files
3. Share link with team
4. Gather feedback

### Medium Term (This Month)
1. Update based on feedback
2. Add analytics (optional)
3. Create promotional materials
4. Share with customers/partners

### Long Term (Ongoing)
1. Add new content as PEGIN evolves
2. Update roadmap quarterly
3. Track wiki analytics
4. Improve based on usage patterns

---

## Best Practices

### Updating Documentation
- ✅ Make updates in markdown first
- ✅ Update HTML if needed
- ✅ Test all links before publishing
- ✅ Commit to git regularly
- ✅ Create release notes for major changes

### Content Organization
- ✅ Group related topics together
- ✅ Use consistent formatting
- ✅ Link between related sections
- ✅ Keep language clear and concise
- ✅ Update examples regularly

### User Experience
- ✅ Keep page load time fast
- ✅ Make search intuitive
- ✅ Provide multiple reading paths
- ✅ Include visual diagrams
- ✅ Add table of contents

### Maintenance
- ✅ Review content quarterly
- ✅ Fix broken links immediately
- ✅ Update outdated information
- ✅ Back up regularly
- ✅ Monitor analytics

---

## Support & Help

### If You Need Help
1. **With HTML wiki:** Try different browser, check JavaScript enabled
2. **With GitHub:** See GitHub Pages documentation
3. **With MkDocs:** See MkDocs documentation
4. **With deployment:** Try Netlify (easier than GitHub Pages)

### Resources
- GitHub Pages Docs: https://pages.github.com
- MkDocs Docs: https://www.mkdocs.org
- Netlify Docs: https://docs.netlify.com
- HTML/CSS Reference: https://developer.mozilla.org

---

## Summary

You now have:

1. **Immediate Solution** ✅
   - Complete HTML wiki (no setup)
   - 18 markdown documents
   - Full documentation

2. **Quick Online Sharing** ⚡
   - GitHub Pages (10 minutes)
   - Netlify (10 minutes)
   - Custom domain (optional)

3. **Professional Solution** 🚀
   - MkDocs site (30 minutes)
   - Search-enabled
   - Mobile-responsive
   - Analytics (optional)

### Choose Your Path:
- **Just want to read?** → Open the HTML file (5 seconds)
- **Want to share?** → Use GitHub Pages (15 minutes)
- **Want professional docs?** → Use MkDocs (30 minutes)

---

**🐧 Start now with whatever option works best for you!**

Next: Open `PEGIN_Wiki_Knowledge_Base.html` in your browser.