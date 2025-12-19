-- Add migration script here
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY,
    actor_id UUID NULL,
    action TEXT NOT NULL,
    resource TEXT NOT NULL,
    metadata JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_audit_logs_actor_id ON audit_logs(actor_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
