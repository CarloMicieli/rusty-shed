#!/usr/bin/env node

/**
 * CSV to JSON Converter
 * 
 * Converts seed CSV files (manufacturers.csv, railway_companies.csv) to JSON
 * for frontend consumption in the SvelteKit application.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**
 * Convert a name to a slug-based ID
 * @param {string} name - The name to convert
 * @returns {string} - Slug ID (lowercase, hyphenated)
 */
function generateSlug(name) {
  return name
    .toLowerCase()
    .replace(/[^\w\s-]/g, '') // Remove special characters
    .replace(/\s+/g, '-')     // Replace spaces with hyphens
    .replace(/-+/g, '-')      // Replace multiple hyphens with single hyphen
    .trim();
}

/**
 * Parse CSV file to JSON array
 * @param {string} csvPath - Path to CSV file
 * @param {function} mapper - Function to map each row to desired structure
 * @returns {Array} - Array of objects
 */
function parseCsv(csvPath, mapper) {
  const content = fs.readFileSync(csvPath, 'utf-8');
  const lines = content.split('\n').filter(line => line.trim());
  
  if (lines.length === 0) {
    throw new Error(`CSV file is empty: ${csvPath}`);
  }

  const headers = lines[0].split(',').map(h => h.trim());
  const data = [];

  for (let i = 1; i < lines.length; i++) {
    const values = lines[i].split(',').map(v => v.trim());
    if (values.length !== headers.length) {
      console.warn(`Skipping malformed line ${i + 1}: ${lines[i]}`);
      continue;
    }

    const row = {};
    headers.forEach((header, index) => {
      row[header] = values[index];
    });

    data.push(mapper(row));
  }

  return data;
}

/**
 * Convert manufacturers CSV to JSON
 */
function convertManufacturers() {
  const csvPath = path.join(__dirname, '../src-tauri/seed/manufacturers.csv');
  const outputPath = path.join(__dirname, '../src/lib/data/manufacturers.json');

  console.log('Converting manufacturers.csv...');

  const manufacturers = parseCsv(csvPath, (row) => ({
    id: generateSlug(row.name),
    name: row.name,
    registered_company_name: row.registered_company_name,
    status: row.status,
    country_code: row.country_code,
    website_url: row.website_url || null
  }));

  // Ensure output directory exists
  const outputDir = path.dirname(outputPath);
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  fs.writeFileSync(outputPath, JSON.stringify(manufacturers, null, 2), 'utf-8');
  console.log(`✓ Created ${outputPath} (${manufacturers.length} manufacturers)`);
}

/**
 * Convert railway companies CSV to JSON
 */
function convertRailwayCompanies() {
  const csvPath = path.join(__dirname, '../src-tauri/seed/railway_companies.csv');
  const outputPath = path.join(__dirname, '../src/lib/data/railway-companies.json');

  console.log('Converting railway_companies.csv...');

  const companies = parseCsv(csvPath, (row) => ({
    id: generateSlug(row.name),
    name: row.name,
    registered_company_name: row.registered_company_name,
    country_code: row.country_code,
    status: row.status,
    operating_since: row.operating_since || null,
    operating_until: row.operating_until || null
  }));

  // Ensure output directory exists
  const outputDir = path.dirname(outputPath);
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  fs.writeFileSync(outputPath, JSON.stringify(companies, null, 2), 'utf-8');
  console.log(`✓ Created ${outputPath} (${companies.length} railway companies)`);
}

// Main execution
try {
  convertManufacturers();
  convertRailwayCompanies();
  console.log('\n✓ All CSV files converted successfully!');
} catch (error) {
  console.error('Error converting CSV files:', error);
  process.exit(1);
}
