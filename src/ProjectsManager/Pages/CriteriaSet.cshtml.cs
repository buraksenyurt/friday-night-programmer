using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using ProjectsManager.Models;
using ProjectsManager.Services;

namespace ProjectsManager.Pages;

public class CriteriaModel(ProjectsApiService apiService) : PageModel
{
    private readonly ProjectsApiService _apiService = apiService;

    [BindProperty]
    public Criteria Criteria { get; set; } = new();

    public List<Criteria> CriteriaSets { get; set; } = [];
    public string ApiMessage { get; set; } = string.Empty;

    public async Task OnGet()
    {
        CriteriaSets = await _apiService.GetAllCriteriaSetsAsync();
    }

    public async Task<IActionResult> OnPostAsync()
    {
        if (!ModelState.IsValid)
        {
            return Page();
        }

        bool success = await _apiService.CreateCriteriaAsync(Criteria);
        if (success)
        {
            TempData["SuccessMessage"] = "Criteria set successfully created!";
            return RedirectToPage("/CriteriaSet");
        }
        else
        {
            ModelState.AddModelError(string.Empty, "Failed to create criteria.");
            CriteriaSets = await _apiService.GetAllCriteriaSetsAsync();
            return Page();
        }
    }

    public async Task<IActionResult> OnPostDeleteCriterionAsync(int setId, string criterionName)
    {
        bool success = await _apiService.DeleteCriterionAsync(setId, criterionName);
        if (success)
        {
            TempData["SuccessMessage"] = "Criterion successfully deleted!";
        }
        else
        {
            TempData["ErrorMessage"] = "Failed to delete criterion.";
        }

        return RedirectToPage("/CriteriaSet");
    }

}
