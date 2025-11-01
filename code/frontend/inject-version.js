#!/usr/bin/env node
import { readFileSync, writeFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Get git hash from PUBLIC_GIT_HASH environment variable or use 'development'
const gitHash = process.env.PUBLIC_GIT_HASH || 'development';

// Read app.html
const appHtmlPath = join(__dirname, 'src', 'app.html');
let html = readFileSync(appHtmlPath, 'utf-8');

// Replace placeholder with actual git hash
html = html.replace('GIT_HASH_PLACEHOLDER', gitHash);

// Write back to app.html
writeFileSync(appHtmlPath, html, 'utf-8');

console.log(`✓ Injected git hash: ${gitHash}`);
