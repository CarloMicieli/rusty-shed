#!/bin/bash

echo "🔍 Running migration validation..."

# 1. Check for old imports
echo "Checking for old shadcn/ imports..."
if grep -r "from '\$lib/components/shadcn" src/; then
  echo "❌ Found old shadcn/ imports"
  exit 1
fi

# 2. Check for old Accordion components
echo "Checking for old Accordion components..."
if grep -r "AccordionItemIndicator\|AccordionItemTrigger\|AccordionItemContent" src/lib/components/ --exclude-dir=ui; then
  echo "❌ Found old Accordion components"
  exit 1
fi

# 3. Check for old ToastHost
echo "Checking for ToastHost usage..."
if grep -r "ToastHost\|ToastProvider" src/; then
  echo "❌ Found old ToastHost/ToastProvider"
  exit 1
fi

# 4. Check for manual validation (no Superforms)
echo "Checking for manual validation patterns..."
if grep -r "let errors = \$state" src/lib/components/; then
  echo "⚠️  Found manual validation patterns (may need review)"
fi

# 5. Run linter
echo "Running linter..."
pnpm lint || exit 1

# 6. Run type checker
echo "Running type checker..."
pnpm check || exit 1

# 7. Run tests
echo "Running tests..."
pnpm test || exit 1

echo "✅ Migration validation complete!"
