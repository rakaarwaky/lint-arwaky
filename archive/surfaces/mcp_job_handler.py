import json
from typing import Any
from ..contract import JobRegistryAggregate, ServiceContainerAggregate


class McpJobCommandsSurface:
    """Surface for MCP job management tools."""

    def __init__(self, mcp: Any = None) -> None:
        self.mcp = mcp
        self.container: ServiceContainerAggregate | None = None

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register the job management tools."""
        self.container = container
        if self.mcp:
            self.mcp.tool()(self.check_status)
            self.mcp.tool()(self.cancel_job)

    async def check_status(self, job_id: str | None = None):
        """Check status of long-running lint jobs."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        job_registry = self.container.get(JobRegistryAggregate)

        if job_id is None:
            all_jobs_vo = await job_registry.list_jobs()
            # Handle both raw dict and MetadataVO
            all_jobs = (
                all_jobs_vo.value if hasattr(all_jobs_vo, "value") else all_jobs_vo
            )
            jobs_list = [
                {"job_id": str(jid), "status": info["status"], "action": info["action"]}
                for jid, info in all_jobs.items()
            ]
            return json.dumps({"jobs": jobs_list, "total": len(jobs_list)})

        job_info_vo = await job_registry.get_job(job_id)
        if job_info_vo is None:
            return json.dumps(
                {"error": f"Job '{job_id}' not found", "status": "not_found"}
            )

        job_info = job_info_vo.value if hasattr(job_info_vo, "value") else job_info_vo
        return json.dumps(
            {
                "job_id": job_id,
                "status": job_info["status"],
                "action": job_info["action"],
                "started_at": job_info.get("started_at"),
                "completed_at": job_info.get("completed_at"),
                "result": job_info.get("result"),
            }
        )

    async def cancel_job(self, job_id: str):
        """Cancel a running lint job."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        job_registry = self.container.get(JobRegistryAggregate)

        from ..taxonomy import JobId

        success = await job_registry.cancel_job(JobId(value=job_id))

        return json.dumps(
            {
                "job_id": job_id,
                "status": "cancelled" if success else "failed_to_cancel",
                "success": bool(success),
            }
        )


def register_job_commands(mcp, container: ServiceContainerAggregate) -> None:
    """Factory function for container-aware registration."""
    surface = McpJobCommandsSurface(mcp)
    surface.register_all(container)


def register_check_status(mcp, container: ServiceContainerAggregate) -> None:
    """Legacy wrapper updated to be container-aware."""
    register_job_commands(mcp, container)


def register_cancel_job(mcp, container: ServiceContainerAggregate) -> None:
    """Legacy wrapper updated to be container-aware."""
    register_job_commands(mcp, container)
