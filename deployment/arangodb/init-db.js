// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
// ArangoDB Initialization Script for Oblibeny BOINC

const db = require('@arangodb').db;
const dbName = 'oblibeny_boinc';

// Create database if it doesn't exist
if (!db._databases().includes(dbName)) {
  db._createDatabase(dbName);
  print(`Database ${dbName} created`);
}

db._useDatabase(dbName);

// Create document collections
const collections = [
  'programs',
  'work_units',
  'results',
  'proofs',
  'properties',
  'volunteers',
  'proof_steps',
  'counterexamples',
  'statistics'
];

collections.forEach(name => {
  if (!db._collection(name)) {
    db._create(name);
    print(`Collection ${name} created`);
  }
});

// Create edge collections for graphs
const edgeCollections = [
  'proof_dependencies',
  'program_variants',
  'property_coverage'
];

edgeCollections.forEach(name => {
  if (!db._collection(name)) {
    db._createEdgeCollection(name);
    print(`Edge collection ${name} created`);
  }
});

// Create graphs
const graphs = require('@arangodb/general-graph');

// Proof dependencies graph
if (!graphs._exists('proof_dependencies')) {
  graphs._create('proof_dependencies', [
    {
      collection: 'proof_dependencies',
      from: ['proofs'],
      to: ['proofs']
    }
  ]);
  print('Graph proof_dependencies created');
}

// Program variants graph (for semantic obfuscation)
if (!graphs._exists('program_variants')) {
  graphs._create('program_variants', [
    {
      collection: 'program_variants',
      from: ['programs'],
      to: ['programs']
    }
  ]);
  print('Graph program_variants created');
}

// Property coverage graph
if (!graphs._exists('property_coverage')) {
  graphs._create('property_coverage', [
    {
      collection: 'property_coverage',
      from: ['programs'],
      to: ['properties']
    }
  ]);
  print('Graph property_coverage created');
}

// Create indexes
db.work_units.ensureIndex({ type: 'persistent', fields: ['status'] });
db.work_units.ensureIndex({ type: 'persistent', fields: ['property_id'] });
db.work_units.ensureIndex({ type: 'persistent', fields: ['created_at'] });

db.results.ensureIndex({ type: 'persistent', fields: ['work_unit_id'] });
db.results.ensureIndex({ type: 'persistent', fields: ['volunteer_id'] });
db.results.ensureIndex({ type: 'persistent', fields: ['validation_status'] });

db.volunteers.ensureIndex({ type: 'persistent', fields: ['reliability_score'] });

db.proofs.ensureIndex({ type: 'persistent', fields: ['property_id'] });
db.proofs.ensureIndex({ type: 'persistent', fields: ['status'] });

print('Indexes created');

// Initialize properties
const properties = [
  {
    _key: '1',
    name: 'Phase Separation Soundness',
    description: 'No compile-time construct appears in deploy-time code',
    status: 'unproven',
    priority: 10,
    formal_statement: '∀ program P, ∀ expression E ∈ deploy-phase(P), compile-only-construct(E) → False'
  },
  {
    _key: '2',
    name: 'Deployment Termination',
    description: 'All deploy-time code provably terminates',
    status: 'unproven',
    priority: 10,
    formal_statement: '∀ program P, ∀ input I, ∃ n ∈ ℕ, eval(deploy-phase(P), I, n) = value'
  },
  {
    _key: '3',
    name: 'Resource Bounds Enforcement',
    description: 'Resource usage never exceeds declared budgets',
    status: 'unproven',
    priority: 9,
    formal_statement: '∀ program P with budget B, ∀ execution trace T, resources(T) ≤ B'
  },
  {
    _key: '4',
    name: 'Capability System Soundness',
    description: 'I/O operations only succeed within capability scope',
    status: 'unproven',
    priority: 9,
    formal_statement: '∀ I/O operation Op, ∀ execution E, success(Op, E) → ∃ capability C ∈ scope(E), allows(C, Op)'
  },
  {
    _key: '5',
    name: 'Obfuscation Semantic Preservation',
    description: 'Code morphing preserves program semantics',
    status: 'unproven',
    priority: 8,
    formal_statement: '∀ program P, ∀ morphed variant P\', ∀ input I, eval(P, I) ≈ eval(P\', I)'
  },
  {
    _key: '6',
    name: 'Call Graph Acyclicity',
    description: 'No recursion in deploy-time code',
    status: 'unproven',
    priority: 10,
    formal_statement: '∀ program P, call-graph(deploy-phase(P)) is acyclic'
  },
  {
    _key: '7',
    name: 'Memory Safety',
    description: 'All memory accesses within bounds',
    status: 'unproven',
    priority: 10,
    formal_statement: '∀ array access A[i] in execution E, 0 ≤ i < length(A)'
  }
];

properties.forEach(prop => {
  try {
    db.properties.insert(prop);
    print(`Property ${prop._key} inserted: ${prop.name}`);
  } catch (e) {
    if (e.errorNum !== 1210) { // Ignore duplicate key errors
      print(`Error inserting property ${prop._key}: ${e.message}`);
    }
  }
});

print('Database initialization complete!');
