exports.transformToResumeFormat = (linkedinData) => {
  return {
    firstName: linkedinData.first_name,
    lastName: linkedinData.last_name,
    jobTitle: linkedinData.job_title,
    themeColor: "#ff6666",
    summery: linkedinData.summary,
    experience: linkedinData.experience.map((exp) => ({
      id: exp.id,
      title: exp.title,
      companyName: exp.company_name,
      startDate: exp.start_date,
      workSummery: exp.work_summary,
      currentlyWorking: !exp.end_date,
    })),
    skills: linkedinData.skills,
  };
};
